// parsentry web UI. Renders API data only via textContent; the bearer token
// lives in sessionStorage so it is dropped when the tab closes.
"use strict";

const TOKEN_KEY = "parsentry.token";
const $ = (id) => document.getElementById(id);

function setStatus(msg) {
  $("status").textContent = msg;
}

async function api(path) {
  const headers = {};
  const token = sessionStorage.getItem(TOKEN_KEY);
  if (token) headers.Authorization = "Bearer " + token;
  const res = await fetch(path, { headers });
  if (res.status === 401) throw new Error("unauthorized: set the API token");
  if (!res.ok) throw new Error(path + ": HTTP " + res.status);
  return res.json();
}

function cell(row, text, cls) {
  const td = document.createElement("td");
  td.textContent = text == null ? "" : String(text);
  if (cls) td.className = cls;
  row.appendChild(td);
}

function clear(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

async function loadRuns() {
  const runs = await api("/api/runs");
  const body = $("runs-body");
  clear(body);
  const select = $("f-run");
  const selected = select.value;
  while (select.options.length > 1) select.remove(1);
  for (const r of runs) {
    const tr = document.createElement("tr");
    cell(tr, r.id);
    cell(tr, r.has_model ? "yes" : "no");
    cell(tr, r.has_report ? "yes" : "no");
    cell(tr, r.surfaces);
    body.appendChild(tr);
    const opt = document.createElement("option");
    opt.value = r.id;
    opt.textContent = r.id;
    select.appendChild(opt);
  }
  select.value = selected;
}

async function loadFindings() {
  const params = new URLSearchParams();
  for (const [key, id] of [["run", "f-run"], ["verdict", "f-verdict"], ["level", "f-level"], ["cwe", "f-cwe"]]) {
    const v = $(id).value.trim();
    if (v) params.set(key, v);
  }
  const qs = params.toString();
  const findings = await api("/api/findings" + (qs ? "?" + qs : ""));
  const body = $("findings-body");
  clear(body);
  for (const f of findings) {
    const tr = document.createElement("tr");
    cell(tr, f.run);
    cell(tr, f.rule_id);
    cell(tr, f.level, f.level === "error" ? "error" : "");
    cell(tr, f.verdict, f.verdict === "confirmed" ? "confirmed" : "");
    cell(tr, (f.cwe || []).join(", "));
    cell(tr, f.file ? f.file + (f.line ? ":" + f.line : "") : "");
    cell(tr, f.message);
    body.appendChild(tr);
  }
}

function run(task) {
  setStatus("");
  task().catch((e) => setStatus(e.message));
}

function show(view) {
  const findings = view === "findings";
  $("runs-view").hidden = findings;
  $("findings-view").hidden = !findings;
  $("tab-runs").classList.toggle("active", !findings);
  $("tab-findings").classList.toggle("active", findings);
  run(findings ? loadFindings : loadRuns);
}

$("tab-runs").addEventListener("click", () => show("runs"));
$("tab-findings").addEventListener("click", () => show("findings"));
$("filters").addEventListener("submit", (e) => {
  e.preventDefault();
  run(loadFindings);
});
$("token-form").addEventListener("submit", (e) => {
  e.preventDefault();
  const token = $("token").value;
  if (token) sessionStorage.setItem(TOKEN_KEY, token);
  else sessionStorage.removeItem(TOKEN_KEY);
  $("token").value = "";
  run(loadRuns);
});

run(loadRuns);
