# 파일명 일괄 변경기 (Creative Namer)

규칙(`타입_구분_서비스_소재명_해상도_YYMM`)에 맞춰 로컬 파일 이름을 일괄 변경하는 도구. 브라우저에서 바로 사용하거나 윈도우·맥 데스크톱 앱으로 설치할 수 있다. 파일은 업로드되지 않으며 모든 처리는 로컬에서 이뤄진다.

## 다운로드

- **웹**: https://hansy-daangn.github.io/CreativeNamer/
- **Windows**: [무설치 실행(CreativeNamer-Run.exe)](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer-Run.exe) · [설치형(CreativeNamer-Setup.exe)](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer-Setup.exe)
- **macOS**: [CreativeNamer.dmg](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer.dmg) (Intel·Apple Silicon 공용)

서명되지 않은 빌드라 최초 실행 시 OS 보호 경고가 표시된다. Windows는 SmartScreen에서 `추가 정보 → 실행`, macOS는 앱을 우클릭 후 `열기`로 한 번 통과시키면 이후에는 표시되지 않는다.

## 사용법

1. 파일을 창에 드래그하거나 파일 선택으로 추가한다.
2. 구분 · 서비스 · 소재명을 지정한다. 하단 미리보기에 결과 이름이 표시된다.
3. `이대로 바꾸기`로 일괄 적용한다. 데스크톱 앱은 권한·저장 확인 없이 제자리에서 즉시 변경한다.

생성 규칙: `타입_구분_서비스_소재명_해상도_YYMM.확장자` — 빈 항목은 생략되며 소재명은 필수다.

## 특징

- 파일 미업로드, 전 과정 로컬 처리.
- 데스크톱 앱은 확인창 없이 제자리 이름 변경(같은 디렉터리 한정, 이름 충돌 시 건너뜀).
- 이미지·영상 해상도를 헤더 파싱으로 자동 입력(mp4·mov·webp·heic 등).
- 소재명 프리셋 공유: 웹은 Supabase 동기화, 데스크톱은 파일 기반 보관/공유.

## 동작 방식

웹과 데스크톱이 동일한 `index.html`을 공유하며 `window.__TAURI__` 유무로 분기한다.

- **웹**: File System Access API로 로컬 파일을 변경하고 소재명 목록을 Supabase에 동기화한다. Chromium 계열(Chrome·Edge)에서 동작한다.
- **데스크톱**: [Tauri](https://tauri.app) v2로 감싸 OS 내장 WebView로 렌더링하고, 파일 입출력은 Rust(`src-tauri/`)가 처리한다. 소재명은 `%APPDATA%\파일명변경기\`에 파일로 보관한다.
  - `materials.jsonl` — 소재명 목록(이름 변경 시 자동 누적)
  - `inbox/` — 공유받은 목록 파일을 두면 다음 실행 시 병합(처리분은 `inbox/_merged/`로 이동)
  - `exports/` — `내보내기`로 생성한 공유 파일

## 빌드 / 배포

- **릴리스**: `v*` 태그를 푸시하면 GitHub Actions가 Windows(`.exe`)·macOS(`.dmg`)를 빌드해 Releases에 게시한다(`workflow_dispatch` 수동 실행은 Artifacts만 산출).
- **웹**: 기본 브랜치 푸시 시 `deploy-pages.yml`로 GitHub Pages에 배포.
- 산출물 `dist/`·`src-tauri/target/`은 `.gitignore` 대상이다.

로컬 빌드(Rust + 플랫폼 WebView 필요):

```bash
mkdir -p dist && cp index.html icon.svg manifest.webmanifest dist/
cd src-tauri && cargo tauri build
```

## 제한

- 되돌리기 미지원. 같은 디렉터리 내에서만 변경하며 이름 충돌 시 건너뛴다.
- 미서명 빌드의 최초 실행 경고(위 참고). 코드 서명 적용 시 제거된다.
- 일부 대용량 영상은 해상도 추출이 누락될 수 있다.
- 데스크톱의 소재명 공유는 파일 기반 수동 동기화다.
