// ============================================================
// CreativeNamer — Figma Plugin (main thread)
//
// 웹·데스크톱과 같은 네이밍 규칙(Supabase naming_rules)으로 선택한 프레임 이름을 바꿉니다.
// UI가 규칙에 따라 이름 조각(parts)을 만들고, 해상도 자리("{res}")만 프레임별 W x H로 채웁니다.
// 이름이 겹치면 _v2, _v3 … 을 붙입니다(웹과 동일).
// ============================================================

figma.showUI(__html__, { width: 340, height: 640, title: "CreativeNamer" });

// 이름을 바꿔줄 대상 노드 타입 (프레임 계열)
var RENAMABLE = {
  FRAME: 1, COMPONENT: 1, COMPONENT_SET: 1, INSTANCE: 1, GROUP: 1, SECTION: 1
};

function yymm() {
  var d = new Date();
  return String(d.getFullYear() % 100).padStart(2, "0") + String(d.getMonth() + 1).padStart(2, "0");
}
function build(parts, sep, w, h) {
  return parts.map(function (p) { return p === "{res}" ? (w + "x" + h) : String(p || "").trim(); })
    .filter(Boolean).join(sep || "_");
}
function targets() {
  return figma.currentPage.selection.filter(function (n) { return RENAMABLE[n.type]; });
}
function sendSelection() {
  var t = targets();
  figma.ui.postMessage({
    type: "selection",
    count: t.length,
    frames: t.map(function (n) { return { w: Math.round(n.width), h: Math.round(n.height) }; }),
    yymm: yymm()
  });
}

figma.ui.onmessage = async function (msg) {
  if (!msg) return;

  // UI 초기화: 로컬 저장 설정 로드 + 현재 선택 전송
  if (msg.type === "init") {
    var store = null;
    try { store = await figma.clientStorage.getAsync("cn_store"); } catch (e) {}
    figma.ui.postMessage({ type: "state", store: store || null });
    sendSelection();
    return;
  }

  // 사용자 설정(커스텀 서비스/구분/접두어/소재명 캐시) 저장
  if (msg.type === "persist") {
    try { await figma.clientStorage.setAsync("cn_store", msg.store); } catch (e) {}
    return;
  }

  // 이름 적용
  if (msg.type === "apply-names") {
    var parts = Array.isArray(msg.parts) ? msg.parts : [];
    var t = targets();
    if (!t.length) { figma.ui.postMessage({ type: "error", text: "프레임을 선택하세요." }); return; }

    var used = {}, names = [];
    for (var i = 0; i < t.length; i++) {
      var n = t[i];
      var base = build(parts, msg.sep, Math.round(n.width), Math.round(n.height)), name = base, d = 2;
      while (used[name.toLowerCase()]) { name = base + "_v" + d; d++; }
      used[name.toLowerCase()] = 1;
      n.name = name;
      names.push(name);
    }
    figma.ui.postMessage({ type: "done", names: names });
    return;
  }

  if (msg.type === "open-url" && msg.url) { figma.openExternal(msg.url); return; }

  if (msg.type === "close") { figma.closePlugin(); }
};

figma.on("selectionchange", sendSelection);
sendSelection();
