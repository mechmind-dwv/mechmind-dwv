// STYLE REMINDER: Neon Matrix Cyber-Core — diagnostic HUD, cyan telemetry, amber thermal warnings, magenta critical state.
import { AlertTriangle, BatteryCharging, Bot, Cable, CheckCircle2, Gauge, RotateCw, Thermometer, Zap } from "lucide-react";
import { Area, AreaChart, Bar, BarChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";

export type ActuatorStatus = "nominal" | "warning" | "critical";

export type ActuatorDiagnostic = {
  id: string;
  name: string;
  powerWatts: number;
  currentAmps: number;
  temperatureC: number;
  status: ActuatorStatus;
};

export type PowerPoint = {
  time: string;
  frontLeft: number;
  frontRight: number;
  rearLeft: number;
  rearRight: number;
  arm: number;
};

export type ThermalEnvelope = {
  coreAvg: number;
  peakMotor: number;
  ambient: number;
};

type DiagnosticsPanelProps = {
  isLive: boolean;
  actuators: ActuatorDiagnostic[];
  powerHistory: PowerPoint[];
  thermalEnvelope: ThermalEnvelope;
  onCommand: (commandType: string) => void;
};

const colors = { frontLeft: "#00f3ff", frontRight: "#79f9ff", rearLeft: "#ffb627", rearRight: "#ff6b35", arm: "#ff0055" };

function statusMeta(status: ActuatorStatus) {
  if (status === "critical") return { label: "critical", className: "diag-critical", Icon: AlertTriangle };
  if (status === "warning") return { label: "warning", className: "diag-warning", Icon: AlertTriangle };
  return { label: "nominal", className: "diag-nominal", Icon: CheckCircle2 };
}

function DiagnosticMetric({ label, value, unit, detail, icon: Icon, tone = "cyan" }: { label: string; value: string; unit: string; detail: string; icon: typeof Gauge; tone?: "cyan" | "amber" | "magenta" }) {
  return <article className={`diagnostic-metric diagnostic-metric-${tone}`}><div className="diagnostic-metric-head"><span>{label}</span><Icon size={15} /></div><div className="diagnostic-metric-value">{value}<small>{unit}</small></div><div className="diagnostic-metric-detail">{detail}</div></article>;
}

export default function DiagnosticsPanel({ isLive, actuators, powerHistory, thermalEnvelope, onCommand }: DiagnosticsPanelProps) {
  const totalPower = actuators.reduce((total, actuator) => total + actuator.powerWatts, 0);
  const warningCount = actuators.filter((actuator) => actuator.status !== "nominal").length;
  const maxPower = Math.max(...actuators.map((actuator) => actuator.powerWatts), 1);

  return <section className="diagnostics-view">
    <header className="diagnostics-header"><div><div className="eyebrow"><span className="eyebrow-line" /> MECH SYSTEMS / ADVANCED DIAGNOSTICS</div><h1>System <em>health.</em></h1><p>Supervisa el reparto de energía, la envolvente térmica y la respuesta de cada actuador antes de enviar una nueva orden autónoma.</p></div><div className={`diagnostics-uplink ${isLive ? "diagnostics-uplink-live" : ""}`}><span className={`status-dot ${isLive ? "dot-green" : "dot-amber"}`} /> {isLive ? "live telemetry" : "local snapshot"}</div></header>

    <div className="diagnostic-metric-grid"><DiagnosticMetric label="Total draw" value={totalPower.toFixed(1)} unit="W" detail="across 5 actuator channels" icon={Zap} /><DiagnosticMetric label="Peak motor" value={thermalEnvelope.peakMotor.toFixed(1)} unit="°C" detail={`${warningCount} channel${warningCount === 1 ? "" : "s"} require attention`} icon={Thermometer} tone="amber" /><DiagnosticMetric label="Core average" value={thermalEnvelope.coreAvg.toFixed(1)} unit="°C" detail={`ambient ${thermalEnvelope.ambient.toFixed(1)}°C`} icon={Gauge} /><DiagnosticMetric label="Power reserve" value="78" unit="%" detail="estimated 18h 42m uptime" icon={BatteryCharging} tone="magenta" /></div>

    <div className="diagnostics-grid"><section className="panel diagnostics-chart-panel"><div className="panel-heading"><div><div className="eyebrow">01 / energy profile</div><h2>Actuator power draw</h2></div><div className="diagnostic-live-label"><span className={`status-dot ${isLive ? "dot-green" : "dot-amber"}`} /> {isLive ? "20Hz stream" : "fallback stream"}</div></div><div className="diagnostic-legend"><span><i style={{ background: colors.frontLeft }} /> front-left</span><span><i style={{ background: colors.rearLeft }} /> rear-left</span><span><i style={{ background: colors.arm }} /> manipulator</span></div><div className="diagnostic-chart"><ResponsiveContainer width="100%" height="100%"><AreaChart data={powerHistory} margin={{ top: 10, right: 10, left: -22, bottom: 0 }}><defs><linearGradient id="powerCyan" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stopColor={colors.frontLeft} stopOpacity={.24} /><stop offset="100%" stopColor={colors.frontLeft} stopOpacity={0} /></linearGradient><linearGradient id="powerMagenta" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stopColor={colors.arm} stopOpacity={.16} /><stop offset="100%" stopColor={colors.arm} stopOpacity={0} /></linearGradient></defs><CartesianGrid stroke="#1c2632" strokeDasharray="2 7" vertical={false} /><XAxis dataKey="time" tick={{ fill: "#657689", fontSize: 10, fontFamily: "JetBrains Mono" }} tickLine={false} axisLine={false} /><YAxis tick={{ fill: "#657689", fontSize: 10, fontFamily: "JetBrains Mono" }} tickLine={false} axisLine={false} /><Tooltip contentStyle={{ background: "#0b1018", border: "1px solid #263747", borderRadius: 0, color: "#eafcff", fontSize: 11 }} labelStyle={{ color: "#00f3ff" }} /><Area type="monotone" dataKey="frontLeft" stroke={colors.frontLeft} fill="url(#powerCyan)" strokeWidth={2} animationDuration={450} /><Area type="monotone" dataKey="rearLeft" stroke={colors.rearLeft} fill="transparent" strokeWidth={1.5} animationDuration={450} /><Area type="monotone" dataKey="arm" stroke={colors.arm} fill="url(#powerMagenta)" strokeWidth={1.5} animationDuration={450} /></AreaChart></ResponsiveContainer></div><div className="diagnostic-footline"><span><Cable size={13} /> /mechros2/system_state</span><span>watts / last 60 min</span></div></section>

      <section className="panel thermal-panel"><div className="panel-heading"><div><div className="eyebrow">02 / thermal envelope</div><h2>Motor temperature</h2></div><Thermometer size={16} className="thermal-heading-icon" /></div><div className="thermal-readout"><strong>{thermalEnvelope.peakMotor.toFixed(1)}°</strong><span>peak / rear-left drive</span></div><div className="thermal-gauge"><span style={{ width: `${Math.min(100, thermalEnvelope.peakMotor / 85 * 100)}%` }} /></div><div className="thermal-scale"><span>0°C</span><span>45°C nominal</span><span>85°C limit</span></div><div className="thermal-meta"><div><span>core average</span><strong>{thermalEnvelope.coreAvg.toFixed(1)}°C</strong></div><div><span>ambient</span><strong>{thermalEnvelope.ambient.toFixed(1)}°C</strong></div></div><div className="thermal-note"><AlertTriangle size={14} /><span>Rear-left drive is above the nominal envelope. Inspect load distribution.</span></div></section>

      <section className="panel actuator-panel"><div className="panel-heading"><div><div className="eyebrow">03 / channel matrix</div><h2>Actuator loadout</h2></div><button className="quiet-button" onClick={() => { if (window.confirm("¿Iniciar calibración de actuadores?")) onCommand("ACTUATOR_CALIBRATION"); }}><RotateCw size={13} /> calibrate</button></div><div className="actuator-list">{actuators.map((actuator) => { const meta = statusMeta(actuator.status); const StatusIcon = meta.Icon; return <div className="actuator-row" key={actuator.id}><div className="actuator-icon"><Bot size={15} /></div><div className="actuator-info"><div><strong>{actuator.name}</strong><span className={`actuator-status ${meta.className}`}><StatusIcon size={11} /> {meta.label}</span></div><div className="actuator-progress"><span style={{ width: `${Math.min(100, actuator.powerWatts / maxPower * 100)}%`, background: actuator.status === "critical" ? "var(--magenta)" : actuator.status === "warning" ? "var(--amber)" : "var(--cyan)" }} /></div></div><div className="actuator-values"><strong>{actuator.powerWatts.toFixed(1)}W</strong><span>{actuator.temperatureC.toFixed(1)}°C / {actuator.currentAmps.toFixed(2)}A</span></div></div>; })}</div></section>

      <section className="panel diagnostic-action-panel"><div className="action-pulse"><div className="pulse-ring" /><div className="pulse-core"><Gauge size={24} /></div></div><div><div className="eyebrow">04 / maintenance protocol</div><h2>Run a full health scan.</h2><p>Capture a synchronized snapshot from every actuator, motor thermal sensor and power rail.</p><button className="command-button primary" onClick={() => onCommand("FULL_HEALTH_SCAN")}><Gauge size={15} /> Run health scan <span>↗</span></button></div></section></div>
  </section>;
}
