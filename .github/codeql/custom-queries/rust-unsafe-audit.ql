// custom-queries/rust-unsafe-audit.ql
import rust

from UnsafeBlock ub
select ub, "¡Código unsafe detectado en MechBot! Revisar ASAP."
