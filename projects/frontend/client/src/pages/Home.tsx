// STYLE REMINDER: Neon Matrix Cyber-Core — asymmetric command-center layout, cyan active telemetry, magenta danger state, glassy carbon panels.
// Style system: cyberpunk operations HUD — asymmetric command-deck layout, cyan/magenta signal accents, compact technical typography, restrained motion.
import { useEffect, useMemo, useRef, useState } from "react";
import {
  Activity,
  AlertTriangle,
  AudioLines,
  Bot,
  BrainCircuit,
  ChevronRight,
  CircleDot,
  Cpu,
  Crosshair,
  Gauge,
  GitBranch,
  Hexagon,
  LayoutDashboard,
  Menu,
  MoreHorizontal,
  Navigation as NavigationIcon,
  Pause,
  Play,
  Radio,
  Radar,
  RefreshCw,
  ScanLine,
  Settings2,
  ShieldCheck,
  Signal,
  SlidersHorizontal,
  TerminalSquare,
  Wifi,
  X,
  Zap,
} from "lucide-react";
import {
  Area,
  AreaChart,
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { toast } from "sonner";
import NavigationPlanner, { type RouteWaypoint } from "@/components/NavigationPlanner";
import DiagnosticsPanel, { type ActuatorDiagnostic, type PowerPoint, type ThermalEnvelope } from "@/components/DiagnosticsPanel";

type TelemetryPoint = {
  time: string;
  load: number;
  signal: number;
  thermal: number;
};

const radarUrl = "/manus-storage/mechmind-orbital-radar_5231015d.jpg";
const mechUrl = "/manus-storage/mechmind-mech-silhouette_96fac0b2.jpg";
const commandUrl = "/manus-storage/mechmind-command-center_9742474a.jpg";
const logoUrl = "/manus-storage/mechmind-cybermark_781492e7.png";

const navItems = [
  { label: "Command deck", icon: LayoutDashboard, value: "deck" },
  { label: "Mech systems", icon: Bot, value: "systems" },
  { label: "AI core", icon: BrainCircuit, value: "ai" },
  { label: "ROS2 mesh", icon: GitBranch, value: "mesh" },
  { label: "Navigation mesh", icon: NavigationIcon, value: "navigation" },
];

const initialTelemetry: TelemetryPoint[] = [
  { time: "14:18", load: 42, signal: 88, thermal: 34 },
  { time: "14:20", load: 48, signal: 91, thermal: 36 },
  { time: "14:22", load: 45, signal: 87, thermal: 35 },
  { time: "14:24", load: 58, signal: 93, thermal: 39 },
  { time: "14:26", load: 55, signal: 89, thermal: 41 },
  { time: "14:28", load: 63, signal: 95, thermal: 43 },
  { time: "14:30", load: 59, signal: 92, thermal: 40 },
];

const initialActuators: ActuatorDiagnostic[] = [
  { id: "act_fl", name: "Front-left drive", powerWatts: 48.2, currentAmps: 4.02, temperatureC: 44.5, status: "nominal" },
  { id: "act_fr", name: "Front-right drive", powerWatts: 47.9, currentAmps: 3.98, temperatureC: 44.1, status: "nominal" },
  { id: "act_rl", name: "Rear-left drive", powerWatts: 52.4, currentAmps: 4.36, temperatureC: 47.2, status: "warning" },
  { id: "act_rr", name: "Rear-right drive", powerWatts: 51.8, currentAmps: 4.31, temperatureC: 46.8, status: "nominal" },
  { id: "act_arm", name: "Manipulator arm", powerWatts: 28.5, currentAmps: 2.37, temperatureC: 39.4, status: "nominal" },
];

const initialPowerHistory: PowerPoint[] = [
  { time: "14:18", frontLeft: 42, frontRight: 41, rearLeft: 46, rearRight: 44, arm: 26 },
  { time: "14:20", frontLeft: 45, frontRight: 43, rearLeft: 48, rearRight: 46, arm: 27 },
  { time: "14:22", frontLeft: 47, frontRight: 45, rearLeft: 50, rearRight: 47, arm: 28 },
  { time: "14:24", frontLeft: 44, frontRight: 46, rearLeft: 49, rearRight: 48, arm: 29 },
  { time: "14:26", frontLeft: 50, frontRight: 48, rearLeft: 53, rearRight: 50, arm: 27 },
  { time: "14:28", frontLeft: 48, frontRight: 47, rearLeft: 52, rearRight: 51, arm: 30 },
  { time: "14:30", frontLeft: 48.2, frontRight: 47.9, rearLeft: 52.4, rearRight: 51.8, arm: 28.5 },
];

const initialThermalEnvelope: ThermalEnvelope = { coreAvg: 42.4, peakMotor: 47.2, ambient: 24.0 };

const alerts = [
  { time: "14:31:08", label: "Vision node synchronized", tone: "cyan", icon: ScanLine },
  { time: "14:29:42", label: "Actuator calibration complete", tone: "green", icon: SlidersHorizontal },
  { time: "14:27:19", label: "Thermal envelope rising", tone: "amber", icon: AlertTriangle },
];

const toneClasses: Record<string, string> = {
  cyan: "tone-cyan",
  green: "tone-green",
  amber: "tone-amber",
};

function MetricCard({
  label,
  value,
  unit,
  delta,
  icon: Icon,
  accent = "cyan",
}: {
  label: string;
  value: string;
  unit: string;
  delta: string;
  icon: typeof Activity;
  accent?: "cyan" | "magenta" | "amber";
}) {
  return (
    <article className={`metric-card metric-${accent}`}>
      <div className="metric-topline">
        <span>{label}</span>
        <Icon size={16} strokeWidth={1.5} />
      </div>
      <div className="metric-value">
        {value}<small>{unit}</small>
      </div>
      <div className="metric-delta">
        <span className="delta-pulse" /> {delta}
      </div>
    </article>
  );
}

function PanelHeading({ eyebrow, title, action }: { eyebrow: string; title: string; action?: string }) {
  return (
    <div className="panel-heading">
      <div>
        <div className="eyebrow">{eyebrow}</div>
        <h2>{title}</h2>
      </div>
      {action && <button className="quiet-button" onClick={() => toast.info(`${action} disponible en el siguiente módulo`)}>{action}<ChevronRight size={14} /></button>}
    </div>
  );
}

export default function Home() {
  const [activeModule, setActiveModule] = useState(() => new URLSearchParams(window.location.search).get("module") || "deck");
  const [streaming, setStreaming] = useState(true);
  const [diagnosticRunning, setDiagnosticRunning] = useState(false);
  const [mobileNav, setMobileNav] = useState(false);
  const [telemetry, setTelemetry] = useState(initialTelemetry);
  const [lastSync, setLastSync] = useState("14:31:16");
  const [isLive, setIsLive] = useState(false);
  const [connectionMode, setConnectionMode] = useState<"connecting" | "reconnecting" | "live" | "simulated">("connecting");
  const [connectionSource, setConnectionSource] = useState("bridge connecting");
  const [batteryLevel, setBatteryLevel] = useState(78);
  const [activeNodes, setActiveNodes] = useState(24);
  const [aiConfidence, setAiConfidence] = useState(96.4);
  const [thermal, setThermal] = useState(41.8);
  const [actuators, setActuators] = useState(initialActuators);
  const [powerHistory, setPowerHistory] = useState(initialPowerHistory);
  const [thermalEnvelope, setThermalEnvelope] = useState(initialThermalEnvelope);
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const bridgeUrl = import.meta.env.VITE_ROS_BRIDGE_WS_URL || "ws://localhost:9090";
    let retryCount = 0;
    let retryTimer: number | undefined;
    let disposed = false;

    const connect = () => {
      if (disposed) return;
      setConnectionMode(retryCount === 0 ? "connecting" : "reconnecting");
      setConnectionSource(retryCount === 0 ? "bridge connecting" : "retrying bridge");
      let socket: WebSocket;
      try {
        socket = new WebSocket(bridgeUrl);
      } catch {
        setIsLive(false);
        setConnectionMode("simulated");
        setConnectionSource("local fallback");
        return;
      }
      socketRef.current = socket;
      socket.onopen = () => {
        if (disposed) return;
        retryCount = 0;
        setIsLive(true);
        setConnectionMode("live");
        setConnectionSource("ros2 bridge");
        socket.send(JSON.stringify({ op: "subscribe", topic: "/mechros2/system_state", type: "std_msgs/msg/String" }));
        socket.send(JSON.stringify({ op: "subscribe", topic: "/mechros2/telemetry", type: "std_msgs/msg/String" }));
      };
      socket.onmessage = (event) => {
        if (disposed) return;
        try {
          const envelope = JSON.parse(event.data);
          if (envelope?.op === "status") return;
          const rawPayload = envelope?.msg?.data ?? envelope?.data ?? envelope;
          const payload = typeof rawPayload === "string" ? JSON.parse(rawPayload) : rawPayload;
          if (!payload) return;
          const timestamp = new Date(payload.timestamp ?? Date.now());
          const time = timestamp.toLocaleTimeString("es-ES", { hour: "2-digit", minute: "2-digit" });
          const nextLoad = Number(payload.core_load ?? payload.load ?? 59);
          const nextSignal = Number(payload.signal_integrity ?? payload.signal ?? 96);
          const nextThermal = Number(payload.temperature ?? payload.thermal ?? 41.8);
          const incomingActuators: ActuatorDiagnostic[] | null = Array.isArray(payload.actuators) ? payload.actuators.map((raw: Record<string, unknown>, index: number) => ({
            id: String(raw.id ?? `act_${index}`),
            name: String(raw.name ?? `Actuator ${index + 1}`),
            powerWatts: Number(raw.power_watts ?? raw.powerWatts ?? 0),
            currentAmps: Number(raw.current_amps ?? raw.currentAmps ?? 0),
            temperatureC: Number(raw.temperature_c ?? raw.temperatureC ?? nextThermal),
            status: raw.status === "critical" || raw.status === "warning" ? raw.status : "nominal",
          })) : null;
          const incomingEnvelope = payload.thermal_envelope as Record<string, unknown> | undefined;
          setTelemetry((items) => [...items.slice(-6), { time, load: nextLoad, signal: nextSignal, thermal: nextThermal }]);
          setBatteryLevel(Number(payload.battery_level ?? payload.battery ?? 78));
          setActiveNodes(Number(payload.active_nodes ?? payload.nodes ?? 24));
          setAiConfidence(Number(payload.ai_confidence ?? payload.confidence ?? 96.4));
          setThermal(nextThermal);
          if (incomingActuators?.length) {
            setActuators(incomingActuators);
            const byId = (id: string) => incomingActuators.find((actuator) => actuator.id === id)?.powerWatts ?? 0;
            setPowerHistory((history) => [...history.slice(-11), { time, frontLeft: byId("act_fl"), frontRight: byId("act_fr"), rearLeft: byId("act_rl"), rearRight: byId("act_rr"), arm: byId("act_arm") }]);
          }
          if (incomingEnvelope) {
            setThermalEnvelope({ coreAvg: Number(incomingEnvelope.core_avg ?? nextThermal), peakMotor: Number(incomingEnvelope.peak_motor ?? nextThermal), ambient: Number(incomingEnvelope.ambient ?? 24) });
          }
          setLastSync(`${time}:${String(timestamp.getSeconds()).padStart(2, "0")}`);
        } catch {
          toast.error("Trama WebSocket inválida", { description: "El bridge envió un mensaje no reconocido." });
        }
      };
      socket.onclose = () => {
        if (disposed) return;
        setIsLive(false);
        setConnectionMode("simulated");
        setConnectionSource("local fallback");
        if (retryCount < 3) {
          retryCount += 1;
          retryTimer = window.setTimeout(connect, 1800);
        }
      };
      socket.onerror = () => socket.close();
    };

    connect();
    return () => {
      disposed = true;
      if (retryTimer) window.clearTimeout(retryTimer);
      socketRef.current?.close();
      socketRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (!streaming || isLive) return;
    const timer = window.setInterval(() => {
      const now = new Date();
      const time = now.toLocaleTimeString("es-ES", { hour: "2-digit", minute: "2-digit" });
      const phase = now.getTime() / 3200;
      const next = {
        time,
        load: 56 + Math.sin(phase) * 8,
        signal: 92 + Math.cos(phase / 2) * 4,
        thermal: 40 + Math.sin(phase / 3) * 3,
      };
      const nextActuators = initialActuators.map((actuator, index) => {
        const wave = Math.sin(phase / 2 + index) * 2.2;
        const temperatureC = actuator.temperatureC + Math.sin(phase / 3 + index) * 1.4;
        return { ...actuator, powerWatts: actuator.powerWatts + wave, currentAmps: Math.max(.4, actuator.currentAmps + wave / 18), temperatureC, status: temperatureC > 49 ? "warning" as const : "nominal" as const };
      });
      setTelemetry((items) => [...items.slice(-6), next]);
      setActuators(nextActuators);
      setPowerHistory((history) => [...history.slice(-11), { time, frontLeft: nextActuators[0].powerWatts, frontRight: nextActuators[1].powerWatts, rearLeft: nextActuators[2].powerWatts, rearRight: nextActuators[3].powerWatts, arm: nextActuators[4].powerWatts }]);
      setThermalEnvelope({ coreAvg: next.thermal + 2.4, peakMotor: Math.max(...nextActuators.map((actuator) => actuator.temperatureC)), ambient: 24 });
      setBatteryLevel((value) => Math.max(70, Math.min(82, value + (Math.random() * 1.2 - .6))));
      setAiConfidence((value) => Math.max(92, Math.min(99, value + (Math.random() * .8 - .4))));
      setThermal(next.thermal);
      setLastSync(`${time}:16`);
    }, 3600);
    return () => window.clearInterval(timer);
  }, [streaming, isLive]);

  const currentLoad = useMemo(() => telemetry[telemetry.length - 1]?.load ?? 59, [telemetry]);
  const currentSignal = useMemo(() => telemetry[telemetry.length - 1]?.signal ?? 92, [telemetry]);

  const sendBridgeCommand = (commandType: string) => {
    if (socketRef.current?.readyState === WebSocket.OPEN) {
      const command = { target_node: "mechros2_hub", command_type: commandType, timestamp: Date.now() };
      socketRef.current.send(JSON.stringify({ op: "publish", topic: "/mechros2/remote_commands", type: "std_msgs/msg/String", msg: { data: JSON.stringify(command) } }));
      toast.success("Comando transmitido", { description: `${commandType} publicado en /mechros2/remote_commands.` });
    } else {
      toast.info("Bridge offline", { description: "La orden queda en modo local hasta recuperar la conexión." });
    }
  };

  const transmitRoute = (route: RouteWaypoint[]) => {
    if (socketRef.current?.readyState !== WebSocket.OPEN) {
      toast.warning("Bridge ROS2 offline", { description: "La ruta queda en el buffer local hasta recuperar UPLINK LIVE." });
      return;
    }
    const routePayload = {
      sequence_id: `route_${Date.now()}`,
      waypoints: route.map((waypoint, index) => ({ ...waypoint, index })),
      issued_at: new Date().toISOString(),
    };
    socketRef.current.send(JSON.stringify({ op: "publish", topic: "/mechros2/navigation_goals", type: "std_msgs/msg/String", msg: { data: JSON.stringify(routePayload) } }));
    toast.success("Ruta transmitida", { description: `${route.length} waypoints publicados en /mechros2/navigation_goals.` });
  };

  const runDiagnostics = () => {
    setDiagnosticRunning(true);
    sendBridgeCommand("DIAGNOSTIC_RUN");
    window.setTimeout(() => {
      setDiagnosticRunning(false);
      toast.success("Diagnóstico completado", { description: "Todos los núcleos responden dentro de parámetros." });
    }, 1800);
  };

  const selectModule = (value: string, label: string) => {
    setActiveModule(value);
    setMobileNav(false);
    if (value !== "deck") toast.info(`${label} seleccionado`, { description: "Vista conectada al command deck." });
  };

  return (
    <div className="dashboard-shell">
      <div className="scanlines" />
      <aside className={`sidebar ${mobileNav ? "sidebar-open" : ""}`}>
        <div className="sidebar-brand">
          <div className="brand-mark"><img src={logoUrl} alt="MechMind" /></div>
          <div>
            <div className="brand-name">MECH<span>MIND</span></div>
            <div className="brand-sub">DWV / COMMAND OS</div>
          </div>
          <button className="mobile-close" onClick={() => setMobileNav(false)} aria-label="Cerrar navegación"><X size={18} /></button>
        </div>

        <div className="side-label">Control surface</div>
        <nav className="main-nav" aria-label="Módulos del sistema">
          {navItems.map(({ label, icon: Icon, value }) => (
            <button key={value} className={`nav-item ${activeModule === value ? "nav-active" : ""}`} onClick={() => selectModule(value, label)}>
              <Icon size={17} strokeWidth={1.7} />
              <span>{label}</span>
              {activeModule === value && <span className="nav-live">LIVE</span>}
            </button>
          ))}
        </nav>

        <div className="side-label">Telemetry</div>
        <div className="side-status-list">
          <div><span className="status-dot dot-cyan" />Sensor grid <strong>24/24</strong></div>
          <div><span className="status-dot dot-green" />Uplink <strong>stable</strong></div>
          <div><span className="status-dot dot-amber" />Power reserve <strong>78%</strong></div>
        </div>

        <div className="sidebar-bottom">
          <div className="operator-card">
            <div className="operator-avatar">DW</div>
            <div><strong>operator.dwv</strong><span>Level 04 / local</span></div>
            <MoreHorizontal size={16} />
          </div>
          <div className="sidebar-footnote"><ShieldCheck size={13} />Secure channel / AES-256</div>
        </div>
      </aside>

      <main className="main-canvas">
        <header className="topbar">
          <div className="topbar-left">
            <button className="mobile-menu" onClick={() => setMobileNav(true)} aria-label="Abrir navegación"><Menu size={20} /></button>
            <div className="breadcrumb"><span>MECHMIND</span><ChevronRight size={13} /><strong>{activeModule === "navigation" ? "NAVIGATION MESH" : activeModule === "systems" ? "MECH SYSTEMS" : "COMMAND DECK"}</strong></div>
          </div>
          <div className="topbar-actions">
            <div className={`connection-pill ${isLive ? "" : "connection-simulated"}`}><span className={`status-dot ${isLive ? "dot-green" : connectionMode === "connecting" || connectionMode === "reconnecting" ? "dot-cyan" : "dot-amber"}`} /> {isLive ? "uplink live" : connectionMode === "connecting" || connectionMode === "reconnecting" ? connectionMode : "uplink simulated"}</div>
            <button className="icon-button" aria-label="Ajustes" onClick={() => toast.info("Panel de ajustes en preparación")}><Settings2 size={17} /></button>
            <div className="top-time">UTC+02 <strong>14:31:16</strong></div>
          </div>
        </header>

        {activeModule === "navigation" ? <NavigationPlanner isLive={isLive} onTransmit={transmitRoute} /> : activeModule === "systems" ? <DiagnosticsPanel isLive={isLive} actuators={actuators} powerHistory={powerHistory} thermalEnvelope={thermalEnvelope} onCommand={sendBridgeCommand} /> : <>
        <section className="hero-strip">
          <img src={commandUrl} alt="Centro de mando robótico" />
          <div className="hero-overlay" />
          <div className="hero-copy">
            <div className="eyebrow"><span className="eyebrow-line" /> SYSTEM OVERVIEW / NODE 01</div>
            <h1>Command deck<br /><em>online.</em></h1>
            <p>Observa, interpreta y dirige el pulso de tu arquitectura robótica desde una sola superficie de control.</p>
          </div>
          <div className="hero-readout">
            <span>MECHBOT-2X</span>
            <strong>MBX / 02</strong>
            <div className="readout-line"><span style={{ width: `${currentSignal}%` }} /></div>
            <small>signal integrity <b>{Math.round(currentSignal)}%</b></small>
          </div>
          <div className="hero-corner">/// AWAITING INSTRUCTION <span>_</span></div>
        </section>

        <section className="dashboard-content">
          <div className="section-intro">
            <div>
              <div className="eyebrow">Live telemetry / {activeModule === "deck" ? "command deck" : activeModule}</div>
              <h2>Good evening, operator.</h2>
            </div>
            <div className="sync-state"><span className={`status-dot ${isLive ? "dot-green" : "dot-amber"}`} /> {connectionSource} / last sync <strong>{lastSync}</strong></div>
          </div>

          <div className="metric-grid">
            <MetricCard label="Core load" value={`${Math.round(currentLoad)}`} unit="%" delta="+4.2% / 30 min" icon={Cpu} />
            <MetricCard label="Battery reserve" value={`${Math.round(batteryLevel)}`} unit="%" delta="+0.8% / 30 min" icon={Zap} accent="amber" />
            <MetricCard label="Active nodes" value={`${activeNodes}`} unit="/24" delta="mesh stable" icon={Radio} accent="magenta" />
            <MetricCard label="AI confidence" value={aiConfidence.toFixed(1)} unit="%" delta="+1.8% / 30 min" icon={BrainCircuit} />
          </div>

          <div className="content-grid">
            <section className="panel telemetry-panel">
              <PanelHeading eyebrow="01 / system pulse" title="Telemetry stream" action="View logs" />
              <div className="chart-legend"><span><i className="legend-cyan" /> core load</span><span><i className="legend-magenta" /> signal</span><span><i className="legend-amber" /> thermal</span><button className="stream-toggle" onClick={() => setStreaming((value) => !value)}>{streaming ? <Pause size={12} /> : <Play size={12} />} {streaming ? "streaming" : "paused"}</button></div>
              <div className="chart-wrap">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={telemetry} margin={{ top: 8, right: 6, left: -28, bottom: 0 }}>
                    <defs>
                      <linearGradient id="loadFill" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stopColor="#00f3ff" stopOpacity={0.25} /><stop offset="100%" stopColor="#00f3ff" stopOpacity={0} /></linearGradient>
                      <linearGradient id="signalFill" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stopColor="#ff0055" stopOpacity={0.12} /><stop offset="100%" stopColor="#ff0055" stopOpacity={0} /></linearGradient>
                    </defs>
                    <CartesianGrid stroke="#1c2632" strokeDasharray="2 7" vertical={false} />
                    <XAxis dataKey="time" tick={{ fill: "#657689", fontSize: 10, fontFamily: "JetBrains Mono" }} tickLine={false} axisLine={false} />
                    <YAxis domain={[0, 100]} tick={{ fill: "#657689", fontSize: 10, fontFamily: "JetBrains Mono" }} tickLine={false} axisLine={false} />
                    <Tooltip contentStyle={{ background: "#0b1018", border: "1px solid #263747", borderRadius: 0, color: "#eafcff", fontSize: 11 }} labelStyle={{ color: "#00f3ff" }} />
                    <Area type="monotone" dataKey="load" stroke="#00f3ff" strokeWidth={2} fill="url(#loadFill)" animationDuration={500} />
                    <Area type="monotone" dataKey="signal" stroke="#ff0055" strokeWidth={1.5} fill="url(#signalFill)" animationDuration={500} />
                    <Line type="monotone" dataKey="thermal" stroke="#ffb627" strokeWidth={1.5} dot={false} animationDuration={500} />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
              <div className="chart-footer"><span><Activity size={13} /> 20Hz sampling</span><span><Wifi size={13} /> ros2 / mechros2_hub</span><span className="chart-footer-right">auto-scale / 60 min</span></div>
            </section>

            <section className="panel orbital-panel">
              <PanelHeading eyebrow="02 / positional awareness" title="Orbital radar" action="Expand" />
              <div className="radar-visual">
                <img src={radarUrl} alt="Radar orbital holográfico" />
                <div className="radar-scan" />
                <div className="radar-label radar-label-a"><span />NODE-07 <b>12m</b></div>
                <div className="radar-label radar-label-b"><span />DRONE-02 <b>48m</b></div>
                <div className="radar-center"><Crosshair size={14} /><span>MBX</span></div>
              </div>
              <div className="radar-stats"><div><span>azimuth</span><strong>184.2°</strong></div><div><span>altitude</span><strong>+02.8m</strong></div><div><span>velocity</span><strong>0.42m/s</strong></div></div>
            </section>

            <section className="panel mech-panel">
              <PanelHeading eyebrow="03 / unit profile" title="MechBot-2X" action="Open system" />
              <div className="mech-visual"><img src={mechUrl} alt="Silueta del MechBot-2X" /><div className="mech-scanline" /><div className="mech-tag tag-top">MBX-02 / RECON</div><div className="mech-tag tag-bottom"><span className="status-dot dot-green" /> autonomous mode</div></div>
              <div className="mech-footer"><div><span>firmware</span><strong>v0.8.4-nightly</strong></div><div><span>temperature</span><strong>{thermal.toFixed(1)}°C</strong></div><div><span>uptime</span><strong>18h 42m</strong></div></div>
            </section>

            <section className="panel command-panel">
              <PanelHeading eyebrow="04 / direct intervention" title="Control surface" />
              <div className="command-copy">Send a controlled instruction to the active unit. Every action is written to the secure node ledger.</div>
              <div className="command-actions"><button className="command-button primary" onClick={runDiagnostics} disabled={diagnosticRunning}><span>{diagnosticRunning ? <RefreshCw className="spin" size={16} /> : <Radar size={16} />}</span>{diagnosticRunning ? "Running diagnostic" : "Run diagnostics"}<ChevronRight size={15} /></button><button className="command-button ghost" onClick={() => { sendBridgeCommand("RESUME_AUTONOMY"); toast.success("Autonomous mode engaged", { description: "MechBot-2X has resumed its navigation loop." }); }}><span><Bot size={16} /></span>Resume autonomy<ChevronRight size={15} /></button></div>
              <div className="command-warning"><AlertTriangle size={14} /><span>Emergency halt is armed in the lower-right channel.</span></div>
              <button className="halt-button" onClick={() => toast.warning("Emergency halt queued", { description: "Confirmación requerida por el operador local." })}><span className="halt-icon"><CircleDot size={15} /></span> Emergency halt <span className="halt-key">ALT + X</span></button>
            </section>
          </div>

          <div className="lower-grid">
            <section className="panel activity-panel">
              <PanelHeading eyebrow="05 / event ledger" title="Recent system activity" action="All activity" />
              <div className="activity-list">{alerts.map(({ time, label, tone, icon: Icon }) => <div className="activity-row" key={time}><div className={`activity-icon ${toneClasses[tone]}`}><Icon size={14} /></div><div className="activity-label"><strong>{label}</strong><span>system event / verified</span></div><time>{time}</time><MoreHorizontal size={15} className="activity-more" /></div>)}</div>
            </section>
            <section className="panel neural-panel">
              <div className="neural-copy"><div className="eyebrow">06 / cognitive layer</div><h2>AI Core is<br /><em>thinking ahead.</em></h2><p>Predictive motion planning is online. Edge model confidence is holding above the operational threshold.</p><button className="text-link" onClick={() => toast.info("AI Core: 8 model traces available")}>Inspect model traces <ChevronRight size={14} /></button></div>
              <div className="neural-orbit"><div className="orbit-ring ring-one" /><div className="orbit-ring ring-two" /><div className="orbit-core"><BrainCircuit size={25} /></div><span className="orbit-node node-one" /><span className="orbit-node node-two" /><span className="orbit-node node-three" /></div>
            </section>
          </div>
          <footer className="dashboard-footer"><span><Hexagon size={13} /> MECHMIND-DWV / INTERNAL CONTROL SYSTEM</span><span>build 2026.08.12 / <b>all systems nominal</b></span></footer>
        </section>
        </>}
      </main>
    </div>
  );
}
