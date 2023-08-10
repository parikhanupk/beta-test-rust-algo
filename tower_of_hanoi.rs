const NUM_DISKS: usize = 3;

static mut NUM_MOVES: u64 = 0;



fn main() {
    // Make three posts with NUM_DISKS entries, all set to 0.
    let mut posts = [[0; NUM_DISKS]; 3];

    // Put the disks on the first post in order, smallest first (on top).
    for i in 0..NUM_DISKS {
        posts[0][i] = i + 1;
    }

    // Draw the initial setup.
    draw_posts(&posts);

    // Move the disks.
    move_disks(&mut posts, NUM_DISKS, 0, 1, 2);
    unsafe {
        println!("Ok, took {NUM_MOVES} moves");
    }
}



// Draw the posts by showing the size of the disk at each level.
fn draw_posts(posts: &[[usize; NUM_DISKS]; 3]) {
    for row in 0..NUM_DISKS {
        // Draw this row.
        for post_num in 0..3 {
            // Draw the disk on post p's row.
            print!("{} ", posts[post_num][row]);
        }
        println!();
    }

    // Draw a line between moves.
    println!("-----");
}



// Move one disk from from_post to to_post.
fn move_disk(posts: &mut [[usize; NUM_DISKS]; 3], from_post: usize, to_post: usize) {
    // Find the first non-empty row in from_post.
    let mut from_row = 0usize;
    for row in 0..NUM_DISKS {
        if posts[from_post][row] != 0 {
            from_row = row;
            break;
        }
    }

    // Find the last empty row in to_post.
    let mut to_row = NUM_DISKS - 1;
    for row in 0..NUM_DISKS {
        if posts[to_post][row] != 0 {
            to_row = row - 1;
            break;
        }
    }

    // Swap the values at those positions.
    (posts[from_post][from_row], posts[to_post][to_row]) =
        (posts[to_post][to_row], posts[from_post][from_row]);
}



// Move the disks from from_post to to_post
// using temp_post as temporary storage.
fn move_disks(
    posts: &mut [[usize; NUM_DISKS]; 3],
    num_to_move: usize,
    from_post: usize,
    to_post: usize,
    temp_post: usize,
) {
    if num_to_move > 0 {
        move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
        move_disk(posts, from_post, to_post);
        unsafe {
            NUM_MOVES += 1;
        }
        draw_posts(posts);
        if num_to_move > 1 {
            move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
        }
    }
}
