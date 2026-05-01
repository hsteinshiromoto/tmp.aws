# Requirements Verification Questions

Please answer the following questions to help clarify the requirements for MealPrep.

## Question 1
What is the target platform for the MVP?

A) REST API only (backend-first, no UI)
B) Full-stack web application (API + web UI)
C) CLI application
D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 2
How should recipe data be stored?

A) In-memory / JSON file (simplest for MVP)
B) DynamoDB (serverless-native, NoSQL)
C) PostgreSQL via RDS/Aurora Serverless
D) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 3
Does the MVP require user authentication?

A) Yes — users must sign up/log in to save their data
B) No — single-user, no auth needed for MVP
C) Basic API key only
D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 4
How should the serverless architecture be deployed?

A) AWS CDK (TypeScript, infrastructure as code)
B) AWS SAM (Serverless Application Model)
C) Serverless Framework
D) Local only for MVP (no cloud deployment yet)
E) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 5
What does "serving size adjustment" mean for the grocery list?

A) Recipes have a default serving size; user picks desired servings and ingredient quantities scale proportionally
B) User sets a household size once and all recipes auto-adjust
C) Both per-recipe adjustment and a global household default
D) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 6
Should the grocery list consolidate duplicate ingredients across meals?

A) Yes — merge identical ingredients and sum quantities (e.g., 2 recipes needing onions = total onions)
B) Yes — merge and also group by store aisle/category
C) No — list ingredients per meal separately
D) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7: Security Extensions
Should security extension rules be enforced for this project?

A) Yes — enforce all SECURITY rules as blocking constraints (recommended for production-grade applications)
B) No — skip all SECURITY rules (suitable for PoCs, prototypes, and experimental projects)
C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8: Property-Based Testing Extension
Should property-based testing (PBT) rules be enforced for this project?

A) Yes — enforce all PBT rules as blocking constraints (recommended for projects with business logic, data transformations, serialization, or stateful components)
B) Partial — enforce PBT rules only for pure functions and serialization round-trips (suitable for projects with limited algorithmic complexity)
C) No — skip all PBT rules (suitable for simple CRUD applications, UI-only projects, or thin integration layers with no significant business logic)
D) Other (please describe after [Answer]: tag below)

[Answer]: A
