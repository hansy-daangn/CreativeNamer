---
name: web-to-windows-app
description: >
  Package this repo's single-file static web tool (index.html — the "파일명 일괄 변경기"
  filename renamer that uses the File System Access API) into a NATIVE WINDOWS app with
  Tauri v2: zero permission/save prompts on rename, real OS drag-drop paths, a server-free
  file-based name database, and a GitHub Actions Windows build that publishes a no-install
  exe + an installer. Use whenever the user asks to "turn the GitHub web app into a Windows
  program / exe", rebuild/refresh the native app, or change its window/build/distribution.
  Do it autonomously end-to-end; the user should not need to re-explain any of this.
---

# web → 네이티브 윈도우 앱 (Tauri v2)

이 스킬은 이 저장소의 `index.html`(브라우저에서 로컬 파일 이름을 바꾸는 정적 웹앱)을
**확인창 없이 즉시 동작하는 윈도우 네이티브 프로그램**으로 만드는 검증된 절차다.
이미 한 번 성공적으로 구축·빌드 검증된 방식이므로, 사용자가 추가 설명을 하지 않아도
이 문서대로 자율적으로 끝까지 만든다.

## 0. 목표(요구사항 고정값)

- UI/UX는 기존 `index.html` 그대로(픽셀 동일). 마크업·CSS는 건드리지 않는다.
- 이름 변경 시 **권한/저장 확인창 0회** — 네이티브 `std::fs::rename`(권한 모델 자체가 없음).
- 어디서 끌어온 파일이든 **자유 드래그드롭**(OS가 주는 절대경로).
- 소재명은 서버 대신 **파일 기반 DB**(`%APPDATA%\파일명변경기\materials.jsonl`) + `inbox` 병합 + 내보내기.
- 결과물: **무설치 실행 exe + 설치 exe**, GitHub Releases로 배포. 빌드는 GitHub Actions(windows-latest).
- 창 기본 크기 460x680, 카드가 창을 꽉 채움(여백 낭비·이중 스크롤·이중 리사이즈 금지).

## 1. 왜 Tauri v2인가 (결정 근거 — 다시 고민하지 말 것)

- WebView2(=윈도우 내장 Chromium)로 기존 HTML을 그대로 렌더 → UI 100% 동일, 포팅 리스크 최소.
- 설치 파일 ~2.5–10MB (Electron은 100–150MB+). 컴맹에게 다운로드 체감이 결정적.
- 드래그드롭 절대경로를 그대로 줌(Electron은 v32에서 `File.path` 제거됨).
- Electron의 유일한 장점(리눅스 크로스빌드)은 우리가 Actions 윈도우 러너로 빌드하므로 무의미.

**금지**: 그냥 PWA/브라우저 유지로는 "확인창 0회"가 구조적으로 불가능(FSA 권한 그랜트 필수). 다시 검토하지 말 것.

## 2. 사전 점검 (가드레일 — 시작 전에 확인)

1. `index.html`이 단일 파일이고 파일 I/O가 한 블록(FSA: `showOpenFilePicker`,
   `getAsFileSystemHandle`, `FileSystemFileHandle.move`, IndexedDB 폴더 핸들)에 모여 있는지 확인.
   흩어져 있으면 먼저 그 지점들을 파악한다(rename / pick / drop / 해상도읽기 / 소재DB).
2. 해상도 추출 파서(`getRes`, `mp4Dim`, `webpDim`, `heifDim`, `imgDim`)는 **Blob만 받으면 동작**한다 →
   그대로 보존하고, 네이티브에선 바이트를 읽어 `new Blob([buf])`로 넘긴다. 절대 재작성하지 말 것.
3. 빌드 호스트가 리눅스여도 됨. 최종 exe는 Actions(windows-latest)가 만든다.
4. 로컬에서 Rust를 타입체크하려면 리눅스용 Tauri 시스템 라이브러리가 필요:
   `apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev librsvg2-dev pkg-config`
   (이건 리눅스 검증 전용. 윈도우 빌드에는 불필요.)

## 3. 파일 구조 (추가/변경물)

```
index.html                      # 단일 소스. window.__TAURI__ 분기만 추가(웹/Pages 그대로 동작)
.gitignore                      # /dist/ , src-tauri/target/ , src-tauri/gen/ , node_modules/
src-tauri/
  Cargo.toml                    # tauri 2, tauri-plugin-dialog 2, serde, serde_json
  build.rs                      # tauri_build::build()
  tauri.conf.json               # frontendDist "../dist", withGlobalTauri true, csp null, 창설정, nsis
  capabilities/default.json     # ["core:default","dialog:default"]
  icons/                        # icon.svg → 생성 (icon.ico 필수)
  src/main.rs                   # 커맨드들
.github/workflows/release.yml   # windows-latest 빌드 → Releases(실행/설치 exe 2개)
dist/                           # 빌드시 생성(gitignore). index.html + icon.svg + manifest 복사본
```

`dist/`는 **frontendDist**가 가리키는 폴더. 깃에 커밋하지 말고 CI(및 로컬 빌드)에서
`index.html`을 복사해 채운다(단일 소스 유지).

## 4. 아이콘 생성 (icon.svg → Tauri 아이콘 세트)

리눅스에 SVG 래스터라이저가 없을 수 있으니 node로 처리:
```bash
cd <scratchpad>; npm init -y >/dev/null; npm i sharp @tauri-apps/cli@^2
node -e "require('sharp')('<repo>/icon.svg',{density:400}).resize(1024,1024).png().toFile('i.png').then(()=>0)"
npx tauri icon i.png -o <repo>/src-tauri/icons
rm -rf <repo>/src-tauri/icons/android <repo>/src-tauri/icons/ios   # 윈도우엔 불필요
```
`tauri.conf.json`의 `bundle.icon`은 최소 `icons/icon.ico` 포함. (없으면 빌드 실패)

## 5. Rust 백엔드 (`src-tauri/src/main.rs`) 핵심 커맨드

- `rename_files(ops: Vec<{path,new_name}>) -> Vec<{path,new_path,from,to,ok,error}>`
  같은 폴더 안 `fs::rename`. **덮어쓰기 금지**(target 존재 시 ok:false). 이름 동일하면 no-op ok:true.
  경로는 `Path`/`OsString`으로 다뤄 한글 파일명 안전.
- `read_file_bytes(path) -> tauri::ipc::Response` 최대 ~100MB 읽어 ArrayBuffer 반환(에러 시 빈 바이트).
  **반드시 `Result<Response>`가 아니라 `Response`를 직접 반환**(매크로 모호성 회피).
- 소재 DB: `load_materials`(inbox 병합 후 dedupe), `add_material`(append 1줄),
  `import_file`(임의 jsonl/txt 병합), `export_materials(filename)`(exports로 복사), `open_data_folder`.
  - 형식: JSON Lines. `{n, service?, series?, division?, ts?}`. JSON이 아닌 줄은 `n`만 있는 이름으로 취급(.txt 호환).
  - dedupe: `n` 소문자 기준, 나중 줄 우선, 최신이 앞.
  - 데이터 폴더: `%APPDATA%\파일명변경기\` (+ `inbox\_merged\`, `exports\`). 업데이트해도 유지됨.
- `fn main()`에서 `.plugin(tauri_plugin_dialog::init())` + `generate_handler![...]` 등록.
- 첫 줄: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` (콘솔창 방지).

## 6. `index.html` 수정 (외과적으로 — UI/CSS/파서 불변)

스크립트 상단에 브리지 추가:
```js
const NATIVE = !!(window.__TAURI__ && window.__TAURI__.core);
const T = NATIVE ? window.__TAURI__ : null;
const nInvoke = (cmd,args)=>T.core.invoke(cmd,args);
```
분기 지점(모두 `if(NATIVE){...}else{<기존 웹 코드>}` 형태로, 웹 경로는 절대 삭제하지 않는다):
1. `SUPPORTED = NATIVE ? true : <기존 FSA 체크>`
2. `addPath(path)` 신설: 절대경로로 파일 추가, `read_file_bytes`→`new Blob([buf])`→`getRes` (파서 그대로).
3. `pickFiles`: 네이티브면 `T.dialog.open({multiple:true})`.
4. 드롭: `setupDrop`(웹, IIFE→함수로) / `setupNativeDrop`(네이티브, `tauri://drag-enter|over|leave|drop`).
   `.jsonl/.txt`를 떨구면 `import_file`로 병합, 그 외는 `addPath`. 초기화에서 `NATIVE?setupNativeDrop():setupDrop()`.
5. `plans()`의 결과에 `path` 추가. `$("go").onclick`에 네이티브 분기: `rename_files` 호출 후 결과로 ok/errs 매핑.
6. `addMat`: 네이티브면 `add_material`도 호출(ts 부여). 초기화에서 `load_materials`로 MATS 교체 + 최초 1회 localStorage 이전.
7. 헤더 `grantCustom` 링크 재활용: 네이티브면 "소재 폴더"(open_data_folder) + "내보내기"(export_materials) 링크.
8. 서비스워커 등록은 `if(!NATIVE && ...)`로 가드.

### 창 꽉 채우기 (이중 스크롤/리사이즈/여백 제거 — 중요)
- CSS에 추가:
  ```css
  body.native{overflow:hidden;}
  body.native .page{padding:0;height:100vh;}
  body.native .card{width:100%;height:100vh;max-width:none;min-width:0;min-height:0;max-height:none;
    border:0;border-radius:0;box-shadow:none;resize:none;}
  ```
- 네이티브 초기화에서 `document.body.classList.add("native")`.
- 창 크기 보정(해상도 감안, try/catch): `T.window.getCurrentWindow().setSize(new T.window.LogicalSize(w,h))`,
  `w=clamp(360..460, availWidth-60)`, `h=clamp(520..680, availHeight-60)`, 후 `center()`.
  (Tauri는 논리 픽셀 → 고DPI에서도 흐릿하지 않음. 작은 화면에서만 축소.)

## 7. `tauri.conf.json` 필수값

- `productName: "파일명 일괄 변경기"`, **`mainBinaryName: "CreativeNamer"`(반드시 ASCII)**, `identifier` ASCII.
- `build.frontendDist: "../dist"`.
- `app.withGlobalTauri: true` (→ `window.__TAURI__.core/event/dialog/window` 노출), `app.security.csp: null`(인라인 스크립트 허용).
- 창: width 460, height 680, minWidth 360, minHeight 520, center true, resizable true, **dragDropEnabled true**.
- `bundle.targets: ["nsis"]`, `bundle.icon`에 `icons/icon.ico` 포함,
  `bundle.windows.nsis.installMode: "currentUser"`(관리자 권한 불필요),
  `bundle.windows.webviewInstallMode.type: "downloadBootstrapper"`(WebView2 자동).

## 8. 빌드 워크플로 (`.github/workflows/release.yml`)

- 트리거: `push.tags: ["v*"]` + `workflow_dispatch`. (검증이 필요하면 임시로 `push.branches: [<작업브랜치>]` 추가 후 검증 끝나면 제거.)
- 단계: checkout → `dist` 채우기(index.html/icon.svg/manifest 복사) → `dtolnay/rust-toolchain@stable`(target x86_64-pc-windows-msvc) → `swatinem/rust-cache`(workspaces: src-tauri) → `tauri-apps/tauri-action@v0`(projectPath ".", **tag일 때만 릴리스**; 여기선 빌드만) →
  결과물을 `out/`에 평평하게 복사하며 친화적 이름 부여(**`파일명변경기_실행.exe`** = 무설치 binary, **`파일명변경기_설치.exe`** = nsis setup) →
  태그면 `softprops/action-gh-release@v2`로 두 파일만 릴리스 → 매 실행 `actions/upload-artifact`(name: windows-app, path: `out/*.exe`).
- binary 이름은 `CreativeNamer.exe`(mainBinaryName) 또는 `creativenamer.exe`(crate명) 둘 다 대비해 복사.
- 산출물은 **딱 2개 평평한 exe**로 유지(폴더 깊이/긴 NSIS 이름 노출 금지).

## 9. 검증 (반드시 수행)

1. JS 문법: `<script>` 추출 후 `node --check`.
2. Rust + 설정 + 권한: 리눅스 시스템 라이브러리 설치 후 `dist/`를 채우고
   `cd src-tauri && cargo build` 성공 확인. **성공 = main.rs 컴파일 + tauri.conf.json 전체 파싱 + capabilities ACL 해석이 모두 유효**(strict 파서라 잘못된 키/권한이면 실패).
3. 윈도우 실제 빌드: 임시 브랜치 트리거로 Actions 실행 → 초록 + `windows-app` Artifacts(2개 exe) 확인 →
   확인 후 임시 트리거 commit으로 제거(같은 commit에서 제거하면 그 push는 다시 빌드되지 않음).
4. 런타임(드래그드롭/다이얼로그/창)은 윈도우에서만 확인 가능 → 사용자에게 Artifacts 다운로드로 테스트 요청.

## 10. 흔한 함정 (반드시 피한다)

- **빌드 산출물 커밋 금지**: `target/`(수백 MB), `dist/`, `gen/`, `node_modules/`는 .gitignore. 커밋 전 `git diff --cached --name-only`로 확인.
- `mainBinaryName`/`identifier`에 한글/공백 금지(크레이트·서명 도구가 깨짐). 표시명은 `productName`/창 `title`에만 한글.
- `frontendDist`가 없으면 `generate_context!`가 panic → 빌드 전 `dist/`를 반드시 생성.
- `read_file_bytes`는 `Response` 직접 반환(`Result<Response>` 아님).
- 거대한 영상은 100MB 캡으로 해상도 못 읽을 수 있음(정상 — 이름변경엔 무관).
- 코드서명 없으면 첫 실행 SmartScreen 1회("추가 정보 → 실행"). README에 스크린샷 안내. 없애려면 OV/EV 서명을 `tauri-action`/`softprops` 단계에 추가(선택).
- 릴리스(공개 게시)는 **사용자 확인 후** `v*` 태그 푸시로 진행(아웃바운드 행위).
- 웹 경로(File System Access)는 절대 삭제하지 말 것(GitHub Pages 버전이 계속 동작해야 함).

## 11. 배포 (사용자에게 안내)

- 테스트용: Actions 실행의 `windows-app` Artifacts(zip 풀면 exe 2개).
- 정식: `git tag vX.Y.Z && git push origin vX.Y.Z` → Releases에 `파일명변경기_실행.exe` + `파일명변경기_설치.exe`.
- 컴맹용 한 줄: "실행.exe 받아서 더블클릭 → 첫 실행 시 추가 정보 → 실행."

## 12. 이 앱에 적용된 추가 UX (유지할 것)

- **README 최상단 퀵스타트**: 1줄 요약 + 직접 다운로드 링크(`releases/latest/download/<한글파일명 percent-encoded>.exe`) + 드래그드롭 사용법, 첫 2~3줄로 받고 쓸 수 있게.
- **창 꽉 채움**: `body.native`에서 `.card`가 창 전체를 채움(여백/이중 스크롤/이중 리사이즈 제거). 기본 460x680 + 해상도 보정 + center.
- **상단 고정 토글**: 헤더에 📌 링크 → `T.window.getCurrentWindow().setAlwaysOnTop(bool)` 토글(`.perm.on` 강조).
- **제목 = 파일 토글**: `+파일선택` 드롭박스 영역 제거(네이티브). 제목 버튼이 `[ ▸ 파일명 일괄 변경기 (N) ]`로 파일 개수 표시 + 클릭 시 목록 패널 토글. 파일 0개면 점선 드래그 안내(`#fileHint`) 표시. 파일명은 가로 스크롤 없이 전체 너비 + ellipsis.
- **소재폴더 vs 내보내기 구분**: `소재폴더`=폴더 열기(`open_data_folder`), `내보내기`=공유파일 생성 후 그 파일을 선택해 보여줌(`reveal_in_folder` = `explorer /select,<path>`). 같은 동작 중복 금지.
- **폴더 안내문**: `ensure_dirs`가 데이터 폴더에 `사용법.txt`(한글 설명)를 1회 생성.
- **기존 소재명 보기 자동 스크롤**: `#sugT`로 열 때 `scrollIntoView({block:'end'})`로 목록까지 자동 스크롤.
- 새 Rust 커맨드는 `generate_handler!`에 반드시 등록(`reveal_in_folder` 포함).
