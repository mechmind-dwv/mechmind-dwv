**Rust Nightly**: Usamos la toolchain específica para MechBot.
**Queries Custom**: 
```ql
   // .github/codeql/custom.qls
   - import: codeql-suites/security-extended.qls
   - queries: ./custom-queries/rust-unsafe-audit.ql
   ```
**Reportes MechMind**: 
   - Hashes de binarios en el summary
   - Categorización especial `/mechmind`
