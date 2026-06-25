# 파일명 일괄 변경기 (Creative Namer)

설치 없이 **웹 주소 접속만으로** 내 컴퓨터의 파일 이름을 규칙에 맞게 일괄 변경하는 정적 웹앱입니다.
파일은 어디에도 업로드되지 않으며, 모든 처리는 브라우저 안에서 로컬로만 일어납니다.

원본 도구(`renamer.bat` + `renamer.ps1`, PowerShell 로컬 서버 방식)를 **브라우저 단독으로 동작하는 순수 웹앱**으로 재구성한 버전입니다.

**접속 주소:** https://hansy-daangn.github.io/CreativeNamer/

## 사용법

1. 위 사이트 주소로 접속합니다.
2. **＋ 파일 선택** 버튼을 누르거나, 파일을 화면에 **드래그앤드롭**합니다.
   - 처음 추가할 때 브라우저가 "파일 수정 허용?"을 **한 번** 물어봅니다. 허용하면 이후엔 추가로 묻지 않습니다.
3. 구분 / 서비스 / 소재명 등을 선택·입력하면 하단 미리보기에 새 파일명이 표시됩니다.
4. **이름 바꾸기** 버튼을 누르면 끝. 추가 클릭 없이 로컬 파일 이름이 바로 바뀝니다.

생성 규칙: `타입_구분_서비스_소재명_해상도_YYMM.확장자` (비어 있는 항목은 자동 생략)

## 동작 원리 / 제약

- **File System Access API** (`showOpenFilePicker`, 드래그앤드롭 `getAsFileSystemHandle`, `FileSystemFileHandle.move()`)를 사용해
  브라우저가 직접 로컬 파일을 제자리에서 이름 변경합니다.
- 해상도(가로x세로)는 이미지/영상 헤더를 브라우저에서 직접 파싱해 채웁니다.
  (mp4·mov·webp·heic 등은 바이트 파서, 그 외 일반 이미지는 브라우저 디코더 사용. 일부 포맷은 해상도를 못 읽을 수 있습니다.)
- **지원 브라우저: Chrome / Microsoft Edge (Chromium 계열).** Firefox·Safari는 이 API를 지원하지 않아 동작하지 않으며,
  접속 시 안내 배너가 표시됩니다.
- 소재명·접두어·구분 프리셋은 브라우저 `localStorage`에 저장됩니다.

## 배포 (GitHub Pages)

이 저장소에는 `.github/workflows/deploy-pages.yml` 가 포함되어 있어, 해당 브랜치에 푸시되면 자동으로 Pages에 배포됩니다.
Pages 소스는 **Settings → Pages → Source: GitHub Actions** 로 설정되어 있어야 합니다.
