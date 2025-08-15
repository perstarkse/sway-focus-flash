use crate::{anim::run_fade_animation_multi, cli::Args};
use swayipc::NodeType;
use swayipc::WindowChange;
use swayipc::{Connection, Event, EventType};
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

pub async fn run_event_loop(args: Args) -> swayipc::Fallible<()> {
    let mut subs = Connection::new()?
        .subscribe(&[EventType::Window, EventType::Workspace])
        .expect("Failed to subscribe IPC");

    let (tx, mut rx) = watch::channel::<Option<i64>>(None);
    let executor_conn = Connection::new()?;

    // Animation manager
    {
        let args_clone = args.clone();
        tokio::spawn(async move {
            let mut prev_focused: Option<i64> = None;
            let mut executor_conn = executor_conn;
            let mut current_cancel = CancellationToken::new();

            while rx.changed().await.is_ok() {
                current_cancel.cancel();
                let new_cancel = CancellationToken::new();
                let focused_id = *rx.borrow();

                if let Some(old) = prev_focused.take() {
                    if Some(old) != focused_id {
                        let _ = executor_conn.run_command(&format!("[con_id={}] opacity 1.0", old));
                    }
                }

                if let Some(new_id) = focused_id {
                    prev_focused = Some(new_id);
                    let args_clone2 = args_clone.clone();
                    let cancel_clone = new_cancel.clone();
                    tokio::spawn(async move {
                        let mut conn = Connection::new().unwrap();
                        run_fade_animation_multi(
                            None,
                            new_id,
                            &mut conn,
                            cancel_clone,
                            args_clone2,
                        )
                        .await
                        .ok();
                    });
                    current_cancel = new_cancel;
                }
            }
        });
    }

    // Event listening
    while let Some(Ok(ev)) = subs.next() {
        match ev {
            Event::Window(win_ev) if win_ev.change == WindowChange::Focus => {
                let id = win_ev.container.id;
                let _ = tx.send(Some(id));
            }
            Event::Workspace(_) => {
                let mut conn = Connection::new()?;
                if let Ok(tree) = conn.get_tree() {
                    fn all_leaves<'a>(node: &'a swayipc::Node, out: &mut Vec<&'a swayipc::Node>) {
                        if node.nodes.is_empty() && node.floating_nodes.is_empty() {
                            out.push(node);
                            return;
                        }
                        for n in &node.nodes {
                            all_leaves(n, out);
                        }
                        for n in &node.floating_nodes {
                            all_leaves(n, out);
                        }
                    }

                    fn focused_leaf_id(node: &swayipc::Node) -> Option<i64> {
                        let mut current = node;
                        loop {
                            if current.nodes.is_empty() && current.floating_nodes.is_empty() {
                                return Some(current.id);
                            }
                            let next = current.focus.first().copied();
                            let Some(next_id) = next else { break };
                            let mut found: Option<&swayipc::Node> = None;
                            for n in &current.nodes {
                                if n.id == next_id {
                                    found = Some(n);
                                    break;
                                }
                            }
                            if found.is_none() {
                                for n in &current.floating_nodes {
                                    if n.id == next_id {
                                        found = Some(n);
                                        break;
                                    }
                                }
                            }
                            if let Some(n) = found {
                                current = n;
                            } else {
                                break;
                            }
                        }
                        None
                    }
                    if let Some(curr_ws) =
                        tree.find(|n| n.focused && n.node_type == NodeType::Workspace)
                    {
                        let mut leaves: Vec<&swayipc::Node> = Vec::new();
                        for n in &curr_ws.nodes {
                            all_leaves(n, &mut leaves);
                        }
                        for n in &curr_ws.floating_nodes {
                            all_leaves(n, &mut leaves);
                        }
                        for win in &leaves {
                            let _ = conn.run_command(&format!("[con_id={}] opacity 1.0", win.id));
                        }
                        if let Some(fid) = focused_leaf_id(&curr_ws) {
                            let _ = tx.send(Some(fid));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
