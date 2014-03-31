// Re-export the sub-libraries under the `units::` namespace
pub use game::units::drawing::{AsGame,AsTile,AsPixel};
pub use game::units::drawing::{Game,Tile,HalfTile,Pixel};

pub use game::units::physics::{min,max};
pub use game::units::physics::{Millis,Velocity,Acceleration};
pub use game::units::physics::{Degrees,AngularVelocity};

pub use game::units::physics::{Frame,Fps};

// Load sub-libraries
pub mod drawing;
pub mod physics;

