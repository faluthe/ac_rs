use crate::{player::Player, process::Process};

use anyhow::Result;

pub unsafe fn run(process: &Process, player1: &mut Player) -> Result<()> {
    // Find the angles to the player with the smallest fov
    let best_angles = process
        .get_players()?
        .into_iter()
        .filter(|player| {
            player.team != player1.team && player.is_alive() && process.is_visible(player1, player)
        })
        .map(|player| player1.angles_to(player))
        .min_by(|a, b| {
            player1
                .view_angles
                .fov_to(a)
                .partial_cmp(&player1.view_angles.fov_to(b))
                .unwrap_or(std::cmp::Ordering::Greater)
        });

    if let Some(best_angles) = best_angles {
        player1.view_angles = best_angles;
    }

    Ok(())
}
