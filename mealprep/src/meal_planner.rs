use crate::models::{Day, MealAssignment, MealPlan};
use crate::storage;
use anyhow::{bail, Result};
use std::path::PathBuf;
use uuid::Uuid;

pub struct MealPlanner {
    plan: MealPlan,
    path: PathBuf,
}

impl MealPlanner {
    pub fn load(path: PathBuf, slots: &[String]) -> Result<Self> {
        let mut plan: MealPlan = storage::load(&path)?;
        if plan.slots.is_empty() {
            plan.slots = slots.to_vec();
        }
        Ok(Self { plan, path })
    }

    fn save(&self) -> Result<()> {
        storage::save(&self.path, &self.plan)
    }

    pub fn set_meal(&mut self, day: Day, slot: &str, recipe_id: Uuid) -> Result<()> {
        let slot_lower = slot.trim().to_lowercase();
        if !self.plan.slots.iter().any(|s| s.to_lowercase() == slot_lower) {
            bail!("Unknown meal slot '{}'. Available: {:?}", slot, self.plan.slots);
        }
        self.plan.assignments.retain(|a| !(a.day == day && a.slot.to_lowercase() == slot_lower));
        self.plan.assignments.push(MealAssignment {
            day,
            slot: slot_lower,
            recipe_id,
        });
        self.save()
    }

    pub fn clear_meal(&mut self, day: Day, slot: &str) -> Result<()> {
        let slot_lower = slot.trim().to_lowercase();
        self.plan.assignments.retain(|a| !(a.day == day && a.slot.to_lowercase() == slot_lower));
        self.save()
    }

    pub fn get_plan(&self) -> &MealPlan {
        &self.plan
    }

    pub fn get_meal(&self, day: Day, slot: &str) -> Option<Uuid> {
        let slot_lower = slot.trim().to_lowercase();
        self.plan
            .assignments
            .iter()
            .find(|a| a.day == day && a.slot.to_lowercase() == slot_lower)
            .map(|a| a.recipe_id)
    }

    pub fn assignments_for_recipe(&self, recipe_id: Uuid) -> Vec<&MealAssignment> {
        self.plan
            .assignments
            .iter()
            .filter(|a| a.recipe_id == recipe_id)
            .collect()
    }

    pub fn remove_recipe(&mut self, recipe_id: Uuid) -> Result<()> {
        self.plan.assignments.retain(|a| a.recipe_id != recipe_id);
        self.save()
    }

    pub fn slots(&self) -> &[String] {
        &self.plan.slots
    }
}
