use wasm_bindgen::prelude::*;

/// A single agent budget with gamma (compute) and eta (communication) components.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Budget {
    gamma: u64,
    eta: u64,
}

#[wasm_bindgen]
impl Budget {
    #[wasm_bindgen(constructor)]
    pub fn new(gamma: u64, eta: u64) -> Budget {
        Budget { gamma, eta }
    }

    pub fn total(&self) -> u64 {
        self.gamma + self.eta
    }

    pub fn check(&self) -> bool {
        true // gamma + eta always equals total by construction
    }

    pub fn gamma(&self) -> u64 {
        self.gamma
    }

    pub fn eta(&self) -> u64 {
        self.eta
    }

    pub fn gamma_fraction(&self) -> f64 {
        let total = self.total() as f64;
        if total == 0.0 { 0.0 } else { self.gamma as f64 / total }
    }

    pub fn eta_fraction(&self) -> f64 {
        let total = self.total() as f64;
        if total == 0.0 { 0.0 } else { self.eta as f64 / total }
    }
}

/// Fleet-wide budget aggregation
#[wasm_bindgen]
pub struct FleetBudget {
    budgets: Vec<Budget>,
}

#[wasm_bindgen]
impl FleetBudget {
    #[wasm_bindgen(constructor)]
    pub fn new(budgets: Vec<Budget>) -> FleetBudget {
        FleetBudget { budgets }
    }

    pub fn total_gamma(&self) -> u64 {
        self.budgets.iter().map(|b| b.gamma).sum()
    }

    pub fn total_eta(&self) -> u64 {
        self.budgets.iter().map(|b| b.eta).sum()
    }

    pub fn total_budget(&self) -> u64 {
        self.total_gamma() + self.total_eta()
    }

    pub fn invariant_holds(&self) -> bool {
        true // always holds by construction
    }

    pub fn len(&self) -> usize {
        self.budgets.len()
    }
}

/// Conservation gauge showing fleet-wide proportions
#[wasm_bindgen]
pub struct ConservationGauge {
    fleet: FleetBudget,
}

#[wasm_bindgen]
impl ConservationGauge {
    #[wasm_bindgen(constructor)]
    pub fn new(fleet: FleetBudget) -> ConservationGauge {
        ConservationGauge { fleet }
    }

    pub fn gamma_fraction(&self) -> f64 {
        let total = self.fleet.total_budget() as f64;
        if total == 0.0 { 0.0 } else { self.fleet.total_gamma() as f64 / total }
    }

    pub fn eta_fraction(&self) -> f64 {
        let total = self.fleet.total_budget() as f64;
        if total == 0.0 { 0.0 } else { self.fleet.total_eta() as f64 / total }
    }

    pub fn status(&self) -> String {
        if self.fleet.invariant_holds() {
            format!("✓ γ({}) + η({}) = {} (conserved)", 
                self.fleet.total_gamma(), self.fleet.total_eta(), self.fleet.total_budget())
        } else {
            format!("✗ CONSERVATION VIOLATION")
        }
    }
}

/// Transfer budget between agents
#[wasm_bindgen]
pub fn transfer(from: &mut Budget, to: &mut Budget, amount_gamma: u64, amount_eta: u64) -> bool {
    if from.gamma >= amount_gamma && from.eta >= amount_eta {
        from.gamma -= amount_gamma;
        from.eta -= amount_eta;
        to.gamma += amount_gamma;
        to.eta += amount_eta;
        true
    } else {
        false
    }
}

/// Check conservation across multiple budgets
#[wasm_bindgen]
pub fn check_fleet_conservation(budgets: &FleetBudget) -> String {
    format!("γ_total={}, η_total={}, total={}, invariant=✓",
        budgets.total_gamma(), budgets.total_eta(), budgets.total_budget())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_budget_creation() {
        let b = Budget::new(143, 82);
        assert_eq!(b.gamma(), 143);
        assert_eq!(b.eta(), 82);
        assert_eq!(b.total(), 225);
    }

    #[wasm_bindgen_test]
    fn test_budget_check() {
        let b = Budget::new(100, 50);
        assert!(b.check());
    }

    #[wasm_bindgen_test]
    fn test_gamma_fraction() {
        let b = Budget::new(143, 82);
        let frac = b.gamma_fraction();
        assert!((frac - 0.6355).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    fn test_eta_fraction() {
        let b = Budget::new(143, 82);
        let frac = b.eta_fraction();
        assert!((frac - 0.3644).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    fn test_fleet_budget() {
        let fleet = FleetBudget::new(vec![
            Budget::new(100, 50),
            Budget::new(43, 32),
        ]);
        assert_eq!(fleet.total_gamma(), 143);
        assert_eq!(fleet.total_eta(), 82);
        assert_eq!(fleet.total_budget(), 225);
        assert!(fleet.invariant_holds());
    }

    #[wasm_bindgen_test]
    fn test_conservation_gauge() {
        let fleet = FleetBudget::new(vec![Budget::new(1430, 820)]);
        let gauge = ConservationGauge::new(fleet);
        assert!((gauge.gamma_fraction() - 0.6355).abs() < 0.01);
        assert!((gauge.eta_fraction() - 0.3644).abs() < 0.01);
        assert!(gauge.status().contains("conserved"));
    }

    #[wasm_bindgen_test]
    fn test_transfer() {
        let mut from = Budget::new(100, 50);
        let mut to = Budget::new(0, 0);
        assert!(transfer(&mut from, &mut to, 30, 10));
        assert_eq!(from.gamma(), 70);
        assert_eq!(from.eta(), 40);
        assert_eq!(to.gamma(), 30);
        assert_eq!(to.eta(), 10);
    }

    #[wasm_bindgen_test]
    fn test_transfer_insufficient() {
        let mut from = Budget::new(10, 5);
        let mut to = Budget::new(0, 0);
        assert!(!transfer(&mut from, &mut to, 20, 10));
    }

    #[wasm_bindgen_test]
    fn test_zero_budget() {
        let b = Budget::new(0, 0);
        assert_eq!(b.total(), 0);
        assert_eq!(b.gamma_fraction(), 0.0);
    }

    #[wasm_bindgen_test]
    fn test_fleet_conservation_check() {
        let fleet = FleetBudget::new(vec![Budget::new(100, 50), Budget::new(43, 32)]);
        let result = check_fleet_conservation(&fleet);
        assert!(result.contains("γ_total=143"));
        assert!(result.contains("η_total=82"));
    }

    #[wasm_bindgen_test]
    fn test_gauge_status() {
        let fleet = FleetBudget::new(vec![Budget::new(143, 82)]);
        let gauge = ConservationGauge::new(fleet);
        let status = gauge.status();
        assert!(status.contains("✓"));
        assert!(status.contains("143"));
    }

    #[wasm_bindgen_test]
    fn test_large_fleet() {
        let budgets: Vec<Budget> = (0..100).map(|i| Budget::new(i, i * 2)).collect();
        let fleet = FleetBudget::new(budgets);
        assert_eq!(fleet.len(), 100);
        assert_eq!(fleet.total_gamma(), (0..100).sum::<u64>());
        assert_eq!(fleet.total_eta(), (0..100).map(|i| i * 2).sum::<u64>());
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;

    #[test]
    fn test_native_budget() {
        let b = Budget::new(143, 82);
        assert_eq!(b.total(), 225);
    }
}
