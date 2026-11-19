"use strict";

const $ = (id) => document.getElementById(id);
const tokenInput = $("token");
tokenInput.value = sessionStorage.getItem("parsentry-token") || "";
tokenInput.addEventListener("change", () => {
  sessionStorage.setItem("parsentry-token", tokenInput.value);
  route();
});

async function api(path) {
  const headers = tokenInput.value ? { Authorization: `Bearer ${tokenInput.value}` } : {};
  const res = await fetch(`/api/${path}`, { headers });
  const body = await res.json();
  if (!res.ok) throw new Error(body.error || res.statusText);
  return body;
}

function row(cells, onClick) {
  const tr = document.createElement("tr");
  for (const [text, cls] of cells) {
    const td = document.createElement("td");
    td.textContent = text ?? "";
    if (cls) td.className = cls;
    tr.appendChild(td);
  }
  if (onClick) tr.addEventListener("click", onClick);
  return tr;
}

async function showRuns() {
  $("runs-view").hidden = false;
  $("findings-view").hidden = true;
  const runs = await api("runs");
  $("runs").replaceChildren(
    ...runs.map((r) =>
      row(
        [[r.id], [r.completed ? "completed" : "in progress"], [String(r.findings)], [r.modified]],
        () => { location.hash = `#/runs/${encodeURIComponent(r.id)}`; },
      ),
    ),
  );
}

async function showFindings(run) {
  $("runs-view").hidden = true;
  $("findings-view").hidden = false;
  $("run-name").textContent = `— ${run}`;
  const params = new URLSearchParams({ run });
  for (const key of ["verdict", "level", "cwe"]) {
    if ($(key).value) params.set(key, $(key).value.trim());
  }
  const findings = await api(`findings?${params}`);
  $("findings").replaceChildren(
    ...findings.map((f) =>
      row([
        [f.rule_id],
        [f.level, `level-${f.level}`],
        [f.verdict],
        [f.cwe.join(", ")],
        [f.location],
        [f.message],
      ]),
    ),
  );
}

async function route() {
  $("error").textContent = "";
  const match = location.hash.match(/^#\/runs\/(.+)$/);
  try {
    if (match) await showFindings(decodeURIComponent(match[1]));
    else await showRuns();
  } catch (e) {
    $("error").textContent = e.message;
  }
}

for (const key of ["verdict", "level", "cwe"]) $(key).addEventListener("change", route);
window.addEventListener("hashchange", route);
route();
