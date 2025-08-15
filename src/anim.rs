use crate::cli::Args;
use swayipc::Connection;
use tokio::time::{Duration, sleep};
use tokio_util::sync::CancellationToken;

/// Runs fade animation on `new_window`. Optionally resets `old_window`.
pub async fn run_fade_animation_multi(
    old_window: Option<i64>,
    new_window: i64,
    conn: &mut Connection,
    token: CancellationToken,
    args: Args,
) -> swayipc::Fallible<()> {
    let step_time = Duration::from_millis(args.frame_time);

    // Start: reset old + set new start opacity in one go
    let mut start_cmd = String::new();
    if let Some(old_id) = old_window {
        start_cmd.push_str(&format!("[con_id={}] opacity 1.0; ", old_id));
    }
    start_cmd.push_str(&format!(
        "[con_id={}] opacity {}",
        new_window, args.start_opacity
    ));
    conn.run_command(&start_cmd)?;

    for i in 0..=args.steps {
        if token.is_cancelled() {
            break;
        }
        let t = i as f32 / args.steps as f32;
        let eased = args.ease.apply(t);
        let opacity = args.start_opacity + (args.end_opacity - args.start_opacity) * eased;
        let cmd = format!("[con_id={}] opacity {:.3}", new_window, opacity);
        conn.run_command(&cmd)?;
        sleep(step_time).await;
    }

    conn.run_command(&format!(
        "[con_id={}] opacity {:.3}",
        new_window, args.end_opacity
    ))?;
    Ok(())
}
