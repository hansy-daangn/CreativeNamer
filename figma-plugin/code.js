// ============================================================
// CreativeNamer — Figma Plugin (main thread)
//
// 웹 '파일명 일괄 변경기'와 동일한 규칙으로 선택한 프레임 이름을 바꿉니다.
//   img_{division}_{service}_{material}_{WxH}_{YYMM}
//   - type       : 항상 img (피그마 프레임 = 이미지 소재)
//   - division   : 구분 (기본 maugrowth)
//   - service    : 서비스 (짧게 표기 옵션 지원)
//   - material   : 소재명 (시리즈 접두어 옵션 지원)
//   - resolution : 프레임 W x H (자동)
//   - date       : YYMM (현재 연월)
// 이름이 겹치면 _v2, _v3 … 을 붙입니다(웹과 동일).
// ============================================================

figma.showUI(__html__, { width: 320, height: 600, title: "CreativeNamer" });

// 이름을 바꿔줄 대상 노드 타입 (프레임 계열)
var RENAMABLE = {
  FRAME: 1, COMPONENT: 1, COMPONENT_SET: 1, INSTANCE: 1, GROUP: 1, SECTION: 1
};
var ORDER = ["type", "division", "service", "material", "resolution", "date"];

function yymm() {
  var d = new Date();
  return String(d.getFullYear() % 100).padStart(2, "0") + String(d.getMonth() + 1).padStart(2, "0");
}
function build(v) {
  return ORDER.map(function (k) { return (v[k] || "").trim(); }).filter(Boolean).join("_");
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
    var p = msg.parts || {};
    var t = targets();
    if (!t.length) { figma.ui.postMessage({ type: "error", text: "프레임을 선택하세요." }); return; }

    var used = {}, names = [];
    for (var i = 0; i < t.length; i++) {
      var n = t[i];
      var vals = {
        type: p.type || "img",
        division: p.division || "",
        service: p.service || "",
        material: p.material || "",
        resolution: Math.round(n.width) + "x" + Math.round(n.height),
        date: p.date || yymm()
      };
      var base = build(vals), name = base, d = 2;
      while (used[name.toLowerCase()]) { name = base + "_v" + d; d++; }
      used[name.toLowerCase()] = 1;
      n.name = name;
      names.push(name);
    }
    figma.ui.postMessage({ type: "done", names: names });
    return;
  }

  if (msg.type === "close") { figma.closePlugin(); }
};

figma.on("selectionchange", sendSelection);
sendSelection();
