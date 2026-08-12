// STYLE REMINDER: Neon Matrix Cyber-Core — tactical map surface, cyan route geometry, magenta selected waypoint, amber transmission gate.
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { ChevronDown, ChevronUp, Crosshair, MapPin, Minus, Navigation, Plus, RotateCcw, Send, Trash2 } from "lucide-react";
import { toast } from "sonner";
import { MapView } from "@/components/Map";

export type WaypointAction = "MOVE" | "SCAN" | "HOLD";

export type RouteWaypoint = {
  id: number;
  lat: number;
  lng: number;
  altitude: number;
  action: WaypointAction;
};

type NavigationPlannerProps = {
  isLive: boolean;
  onTransmit: (waypoints: RouteWaypoint[]) => void;
};

const initialCenter = { lat: 40.7128, lng: -74.006 };

function distanceBetween(a: RouteWaypoint, b: RouteWaypoint) {
  const earthRadius = 6371000;
  const lat1 = (a.lat * Math.PI) / 180;
  const lat2 = (b.lat * Math.PI) / 180;
  const deltaLat = ((b.lat - a.lat) * Math.PI) / 180;
  const deltaLng = ((b.lng - a.lng) * Math.PI) / 180;
  const haversine = Math.sin(deltaLat / 2) ** 2 + Math.cos(lat1) * Math.cos(lat2) * Math.sin(deltaLng / 2) ** 2;
  return earthRadius * 2 * Math.atan2(Math.sqrt(haversine), Math.sqrt(1 - haversine));
}

export default function NavigationPlanner({ isLive, onTransmit }: NavigationPlannerProps) {
  const [waypoints, setWaypoints] = useState<RouteWaypoint[]>([]);
  const [selectedId, setSelectedId] = useState<number | null>(null);
  const [mapReady, setMapReady] = useState(false);
  const mapRef = useRef<google.maps.Map | null>(null);
  const polylineRef = useRef<google.maps.Polyline | null>(null);
  const markersRef = useRef<any[]>([]);
  const mapListenerRef = useRef<google.maps.MapsEventListener | null>(null);
  const nextId = useRef(1);

  const addWaypoint = useCallback((lat: number, lng: number) => {
    const id = nextId.current++;
    setWaypoints((current) => [...current, { id, lat, lng, altitude: 2.5, action: "MOVE" }]);
    setSelectedId(id);
    toast.success(`Waypoint ${id} añadido`, { description: "Ajusta la acción o altitud en el panel lateral." });
  }, []);

  const handleMapReady = useCallback((map: google.maps.Map) => {
    mapRef.current = map;
    setMapReady(true);
    mapListenerRef.current?.remove();
    mapListenerRef.current = map.addListener("click", (event: google.maps.MapMouseEvent) => {
      if (event.latLng) addWaypoint(event.latLng.lat(), event.latLng.lng());
    });
  }, [addWaypoint]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || typeof google === "undefined") return;

    markersRef.current.forEach((marker) => { marker.map = null; });
    markersRef.current = waypoints.map((point, index) => {
      const markerElement = document.createElement("div");
      markerElement.className = `planner-marker ${selectedId === point.id ? "planner-marker-selected" : ""}`;
      markerElement.innerHTML = `<span>${index + 1}</span>`;
      markerElement.addEventListener("click", () => setSelectedId(point.id));
      return new google.maps.marker.AdvancedMarkerElement({
        map,
        position: { lat: point.lat, lng: point.lng },
        title: `Waypoint ${index + 1}`,
        content: markerElement,
      });
    });

    polylineRef.current?.setMap(null);
    polylineRef.current = new google.maps.Polyline({
      map,
      path: waypoints.map(({ lat, lng }) => ({ lat, lng })),
      strokeColor: "#00f3ff",
      strokeOpacity: 0.9,
      strokeWeight: 2,
      icons: [{ icon: { path: "M 0,-1 0,1", strokeOpacity: 1, scale: 3 }, offset: "0", repeat: "18px" }],
    });
  }, [waypoints, selectedId]);

  useEffect(() => () => {
    mapListenerRef.current?.remove();
    markersRef.current.forEach((marker) => { marker.map = null; });
    polylineRef.current?.setMap(null);
  }, []);

  const totalDistance = useMemo(() => waypoints.slice(1).reduce((total, waypoint, index) => total + distanceBetween(waypoints[index], waypoint), 0), [waypoints]);
  const selectedWaypoint = waypoints.find((waypoint) => waypoint.id === selectedId);

  const updateWaypoint = (id: number, patch: Partial<RouteWaypoint>) => {
    setWaypoints((current) => current.map((waypoint) => waypoint.id === id ? { ...waypoint, ...patch } : waypoint));
  };

  const moveWaypoint = (id: number, direction: -1 | 1) => {
    setWaypoints((current) => {
      const index = current.findIndex((waypoint) => waypoint.id === id);
      const target = index + direction;
      if (index < 0 || target < 0 || target >= current.length) return current;
      const reordered = [...current];
      [reordered[index], reordered[target]] = [reordered[target], reordered[index]];
      return reordered;
    });
  };

  const removeWaypoint = (id: number) => {
    setWaypoints((current) => current.filter((waypoint) => waypoint.id !== id));
    setSelectedId(null);
  };

  const handleFallbackMapClick = (event: React.MouseEvent<HTMLDivElement>) => {
    if (mapReady) return;
    const bounds = event.currentTarget.getBoundingClientRect();
    const x = (event.clientX - bounds.left) / bounds.width;
    const y = (event.clientY - bounds.top) / bounds.height;
    addWaypoint(initialCenter.lat + (0.5 - y) * 0.04, initialCenter.lng + (x - 0.5) * 0.06);
  };

  const clearRoute = () => {
    if (waypoints.length && !window.confirm("¿Eliminar todos los waypoints de esta ruta?")) return;
    setWaypoints([]);
    setSelectedId(null);
  };

  const transmitRoute = () => {
    if (waypoints.length < 2) {
      toast.warning("Ruta incompleta", { description: "Añade al menos dos waypoints antes de transmitir." });
      return;
    }
    if (!isLive) {
      toast.warning("Bridge ROS2 offline", { description: "La ruta se mantiene local hasta recuperar UPLINK LIVE." });
      return;
    }
    if (!window.confirm(`Transmitir ${waypoints.length} waypoints al MechBot-2X?`)) return;
    onTransmit(waypoints);
  };

  return (
    <section className="navigation-planner-view">
      <div className="planner-header">
        <div>
          <div className="eyebrow"><span className="eyebrow-line" /> ROS2 MESH / NAVIGATION PLANNER</div>
          <h1>Route <em>editor.</em></h1>
          <p>Define una trayectoria autónoma, revisa sus acciones y envíala al MechBot-2X cuando el enlace seguro esté disponible.</p>
        </div>
        <div className={`planner-uplink ${isLive ? "planner-uplink-live" : ""}`}><span className={`status-dot ${isLive ? "dot-green" : "dot-amber"}`} /> {isLive ? "uplink live" : "uplink simulated"}</div>
      </div>

      <div className="planner-layout">
        <div className="planner-map-panel">
          <div className="planner-map-toolbar"><div><span className="eyebrow">TACTICAL MAP / NODE 01</span><strong>Click on map to place waypoint</strong></div><div className="planner-toolbar-actions"><button onClick={() => mapRef.current?.setZoom(Math.min((mapRef.current?.getZoom() ?? 12) + 1, 20))} aria-label="Acercar mapa"><Plus size={15} /></button><button onClick={() => mapRef.current?.setZoom(Math.max((mapRef.current?.getZoom() ?? 12) - 1, 3))} aria-label="Alejar mapa"><Minus size={15} /></button><button onClick={() => mapRef.current?.panTo(initialCenter)} aria-label="Centrar mapa"><Crosshair size={15} /></button></div></div>
          <div className="planner-map-shell">
            <MapView className="planner-google-map" initialCenter={initialCenter} initialZoom={13} onMapReady={handleMapReady} />
            <div className={`planner-map-fallback ${mapReady ? "" : "map-fallback-active"}`} onClick={handleFallbackMapClick}><div className="fallback-grid" /><span className="fallback-label fallback-a">SECTOR 07 / URBAN MESH</span><span className="fallback-label fallback-b">{mapReady ? "GPS LINK / LIVE" : "GPS LINK / TACTICAL FALLBACK"}</span><div className="fallback-crosshair"><Crosshair size={20} /></div></div>
            <div className="planner-map-legend"><span><i className="legend-route" /> route line</span><span><i className="legend-mech" /> mechbot position</span><span><i className="legend-wp" /> waypoint</span></div>
          </div>
        </div>

        <aside className="planner-sidebar">
          <div className="planner-side-head"><div><div className="eyebrow">ROUTE BUFFER</div><h2>Waypoints <b>{waypoints.length.toString().padStart(2, "0")}</b></h2></div><button className="quiet-button" onClick={clearRoute}><RotateCcw size={13} /> reset</button></div>
          <div className="planner-stats"><div><span>distance</span><strong>{totalDistance > 1000 ? `${(totalDistance / 1000).toFixed(2)}km` : `${Math.round(totalDistance)}m`}</strong></div><div><span>altitude</span><strong>{selectedWaypoint?.altitude.toFixed(1) ?? "—"}m</strong></div><div><span>sequence</span><strong>{waypoints.length ? "READY" : "EMPTY"}</strong></div></div>

          <div className="waypoint-list">
            {waypoints.length === 0 ? <div className="waypoint-empty"><Navigation size={24} /><strong>Route buffer empty</strong><span>Click anywhere on the map to place your first waypoint.</span></div> : waypoints.map((waypoint, index) => <div key={waypoint.id} className={`waypoint-card ${selectedId === waypoint.id ? "waypoint-selected" : ""}`} onClick={() => setSelectedId(waypoint.id)}><div className="waypoint-number">{String(index + 1).padStart(2, "0")}</div><div className="waypoint-main"><div className="waypoint-title"><strong>WP-{String(index + 1).padStart(2, "0")}</strong><select value={waypoint.action} onChange={(event) => updateWaypoint(waypoint.id, { action: event.target.value as WaypointAction })} onClick={(event) => event.stopPropagation()}><option value="MOVE">MOVE</option><option value="SCAN">SCAN</option><option value="HOLD">HOLD</option></select></div><div className="waypoint-coords">{waypoint.lat.toFixed(5)} / {waypoint.lng.toFixed(5)}</div><label className="altitude-field">ALT <input type="number" min="0" max="50" step="0.1" value={waypoint.altitude} onChange={(event) => updateWaypoint(waypoint.id, { altitude: Number(event.target.value) })} onClick={(event) => event.stopPropagation()} /> m</label></div><div className="waypoint-actions"><button onClick={(event) => { event.stopPropagation(); moveWaypoint(waypoint.id, -1); }} aria-label="Subir waypoint"><ChevronUp size={13} /></button><button onClick={(event) => { event.stopPropagation(); moveWaypoint(waypoint.id, 1); }} aria-label="Bajar waypoint"><ChevronDown size={13} /></button><button className="delete-waypoint" onClick={(event) => { event.stopPropagation(); removeWaypoint(waypoint.id); }} aria-label="Eliminar waypoint"><Trash2 size={13} /></button></div></div>)}
          </div>

          <div className="planner-footer"><button className="planner-transmit" onClick={transmitRoute} disabled={waypoints.length < 2}><Send size={15} /> Transmit route <span>↗</span></button><div className="planner-topic"><span className={`status-dot ${isLive ? "dot-green" : "dot-amber"}`} /> /mechros2/navigation_goals</div></div>
        </aside>
      </div>
    </section>
  );
}
