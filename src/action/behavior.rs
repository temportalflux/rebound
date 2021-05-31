pub static ACTION_BEHAVIOR_DEFAULT_BUTTON: Behavior = Behavior { digital_axis: None };

#[derive(Debug, Clone)]
pub struct Behavior {
	digital_axis: Option<DigitalAxis>,
}

impl Behavior {
	pub(crate) fn _digital_axis(&self) -> &Option<DigitalAxis> {
		&self.digital_axis
	}
}

/// For [`Button`](crate::source::Kind::Button) events bound to [`Axis`](crate::source::Kind::Axis) actions.
#[derive(Debug, Clone)]
pub struct DigitalAxis {
	reverse: Option<DigitalAxisReverse>,
	/// Speed (units/sec) that the axis value falls toward 0.
	gravity: f32,
	/// Speed to move toward an axis value of 1.0 in units/sec.
	sensitivity: f32,
}

/// Modifier applied when input is received in the opposite direction of the current flow.
#[derive(Debug, Clone)]
pub enum DigitalAxisReverse {
	/// Snap axis value to 0 and continue from there.
	Snap,
	/// Reverse the current value to the opposite sign and continue from there.
	InstantReverse,
}

impl Default for Behavior {
	fn default() -> Self {
		Self { digital_axis: None }
	}
}
