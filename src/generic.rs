pub trait GameInfo: Clone {}

pub trait GameState: Clone {}

pub trait TurnPlan: Clone {}


///TODO: MOVE THIS TO ANOTHER/MORE GENERIC CRATE ALONG WITH SOME OTHER STUFF AS A GENERIC BASE READY FOR FANTASTIC BITS
///empty atm supposed to be a base type for AIs such that they can verse themselves or a simpler enemys

pub trait AI<'a, Info,State,Plan>where Info: GameInfo, State: GameState, Plan: TurnPlan {
	fn generate_action(&mut self, state: & State) -> Plan;
}
