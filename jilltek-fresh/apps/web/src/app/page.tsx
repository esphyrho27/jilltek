const modules = [
  ["Network Operations", "Topology, nodes, ports and live status"],
  ["Customers & Services", "Customer records, plans and subscriptions"],
  ["Monitoring", "Router telemetry and historical metrics"],
  ["Billing & Payments", "Bills, balances and payment records"],
  ["Dispatch & Installations", "Technician jobs and service activation"],
  ["Reports & Audit", "Operational reporting and traceability"]
];
export default function Home() {
  return <main className="shell">
    <header><div className="mark">J</div><div><small>JILLTEK OPERATIONS</small><h1>ISP Management Platform</h1></div><span className="tag">FRESH FOUNDATION</span></header>
    <section className="hero"><small>NETWORK CONTROL CENTER</small><h2>One platform for your entire network.</h2><p>A clean foundation for network operations, subscriber services, monitoring, billing and field work.</p><b>Rust · Next.js · PostgreSQL</b></section>
    <section className="heading"><div><small>WORKSPACES</small><h2>Platform modules</h2></div><span>06 MODULES</span></section>
    <section className="grid">{modules.map(([name, detail], i) => <article key={name}><small>0{i+1} · ROADMAP</small><h3>{name}</h3><p>{detail}</p><footer>MODULE IN ROADMAP <b>↗</b></footer></article>)}</section>
    <footer className="bottom">JILLTEK ISP MANAGEMENT <span>Fresh architecture foundation · v0.1.0</span></footer>
  </main>;
}
