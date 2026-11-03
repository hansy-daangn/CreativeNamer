# CreativeNamer — Figma 플러그인

웹·데스크톱 CreativeNamer와 **같은 네이밍 규칙**으로 선택한 프레임 이름을 한 번에 바꿉니다.

- 규칙(항목 순서·이름·값·약어)은 서버(Supabase `naming_rules`)의 최신본을 열 때마다 불러와요.
- 규칙을 바꾸려면 플러그인의 **규칙 편집 ↗** 을 눌러 웹의 [네이밍 규칙 탭](https://hansy-daangn.github.io/CreativeNamer/#rules)에서 수정·저장하세요.
- 유형은 항상 이미지 코드(기본 `image`), 해상도는 프레임 `가로x세로`, 날짜는 오늘 기준으로 자동 입력돼요.
- 결과 이름이 겹치면 `_v2`, `_v3` … 을 붙여요 (웹과 동일).
- 서버에 연결할 수 없으면 마지막으로 받은 규칙으로 그대로 동작해요.

```
기본 규칙 예) image_maugrowth_fleamarket_keyword-spring_1080x1080_2611
```

## 설치

1. 이 폴더의 `manifest.json`, `code.js`, `ui.html` 을 한 폴더에 내려받아요.
2. Figma 데스크톱 앱 → **Plugins → Development → Import plugin from manifest…**
3. `manifest.json` 을 선택 → **Plugins → Development → CreativeNamer** 로 실행.

## 사용

1. 이름을 바꿀 프레임(들)을 선택해요.
2. 규칙에 있는 항목(구분 · 서비스 · 소재명 등)을 지정해요.
3. **이름 바꾸기** 를 누르면 선택한 프레임 이름이 한 번에 바뀝니다.

## 구성

| 파일 | 역할 |
| --- | --- |
| `manifest.json` | 플러그인 메타/네트워크 권한 |
| `code.js` | 메인 스레드 — 선택 감지, 프레임 이름 적용, 로컬 설정 저장(`clientStorage`) |
| `ui.html` | 패널 UI + 네이밍/미리보기/Supabase 동기화 로직 |

> 수정 요청은 마케팅팀 hansy에게 알려주세요.
