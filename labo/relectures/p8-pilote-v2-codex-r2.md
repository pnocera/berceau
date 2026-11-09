Scoped P8 re-confirmation: read the earlier review and inspected only the pilot diff in labo/src/main.rs and the Learner doc comment in labo/src/pilote.rs. No repository files were modified.

F1 resolved. pilot_rows and pilot_summary emit separate W/L fields using case.scale and case.rule, with matching W,L headers (main.rs:350–391). cargo build --release and the fresh pilote-smoke run succeeded; output: /tmp/codex-pilote-smoke2-385279. Python's strict csv.reader and csv.DictReader verified exploration.csv: 64 data rows, 14 fields in the header and every row; exploration-summary.csv: four data rows, 11 fields in the header and every row. All four W/L pairs parse correctly, each has roots 900–915, numeric columns parse, and summary means match their corresponding replicate metrics to printed precision.

validation.csv uses PILOT_ROWS_HEADER and pilot_rows; validation-summary.csv uses PILOT_SUMMARY_HEADER and pilot_summary (main.rs:467–471), so both share the fixed writers. This was confirmed by source inspection: the smoke run selected no candidate and therefore produced no validation CSVs or validation-root evaluations. Only smoke roots 900–915 were executed; labo pilote was never run and roots 100–131 were not accessed.

F2 documented at pilote.rs:145–148: the embedded Organism retains birth baseline/utilities/age, only its weights evolve, and the comment explicitly warns against serialization or invoking its learning/renewal methods. No blockers remain within this re-confirmation scope.

VERDICT: GO
