@startuml
component "MechBot CPU" as core
component "Sensors" as sensors
core --> sensors : I2C
@enduml
