# 파일명 일괄 변경기 (Creative Namer)
파일을 창에 드래그하면 이름을 바꿔줘요: 네이밍 컨벤션(`타입_구분_서비스_소재명_해상도_YYMM`)

- [**브라우저**](https://hansy-daangn.github.io/CreativeNamer/), [**윈도우**](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer-Setup.exe), [**맥OS**](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer.dmg)를 지원하며, 로컬로만 구동되어 보안성과 작업 속도를 극대화해요.
- 브라우저는 링크만 알면 사용할 수 있지만, 보안정책상 파일명을 바꿀때마다 확인을 눌러줘야 해요.
- 윈도우는 [**무설치 실행파일**](https://github.com/hansy-daangn/CreativeNamer/releases/latest/download/CreativeNamer-Run.exe)로도 사용할 수 있어요.


## 사용하기

1. 파일을 창에 드래그해요.
2. 구분 · 서비스 · 소재명을 지정해요. 시리즈 소재의 경우, 접두어를 추가할 수 있어요.
3. `바꾸기`버튼을 클릭하면, 일괄 적용돼요.

생성 규칙: `타입_구분_서비스_소재명_해상도_YYMM.확장자` — 빈 항목은 생략되며 소재명은 필수다.


## 특징

- 소재명은 자동으로 저장돼요: 웹은 Supabase서버 동기화, 데스크톱은 파일 기반 저장/공유.
- 이미지·영상 해상도를 헤더 파싱으로 자동 입력(mp4·mov·webp·heic 등).


<details><summary>상세 정보</summary>

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
- 일부 대용량 영상은 해상도 추출이 누락될 수 있다.
- 데스크톱의 소재명 공유는 파일 기반 수동 동기화다.
</details>
