use bevy::prelude::*;

#[derive(Debug, Component)]
pub(crate) struct MessageBoard;

#[derive(Debug, Event)]
pub(crate) struct MessageBoardEvent(pub(crate) String);

#[macro_export]
macro_rules! post {
    ($board:expr, $($message:tt)*) => {
        $board.send(MessageBoardEvent(format!($($message)*)))
    };
}

pub(crate) fn update_message_board(
    mut message_board_posts: EventReader<MessageBoardEvent>,
    mut message_board_query: Query<&mut Text, With<MessageBoard>>,
) {
    message_board_posts
        .read()
        .for_each(|MessageBoardEvent(post)| {
            message_board_query.single_mut().0 = post.clone();
        });
}
