# CreativeNamer — Figma 플러그인

웹 '파일명 일괄 변경기'와 **동일한 규칙**으로 선택한 프레임 이름을 한 번에 바꿉니다.

```
img_{구분}_{서비스}_{소재명}_{가로x세로}_{YYMM}
예) img_maugrowth_fleamarket_keyword-spring_1080x1080_2607
```

- `type` : 항상 `img` (피그마 프레임 = 이미지 소재)
- `구분(division)` : 기본 `maugrowth`, 직접 추가 가능
- `서비스(service)` : 중고거래/알바/모임/부동산/중고차/비즈니스 + '짧게 표기해요' 옵션(예: `fleamarket`→`Fl`)
- `소재명(material)` : 자유 입력. '시리즈 소재예요'를 켜면 접두어(`promo`, `keyword` …)를 붙일 수 있어요 (`keyword-spring`)
- `해상도` : 선택한 프레임의 `가로x세로` 자동 입력
- `YYMM` : 현재 연월 자동 입력
- 결과 이름이 겹치면 `_v2`, `_v3` … 을 붙여요 (웹과 동일)

소재명 목록은 웹·데스크톱 버전과 **같은 Supabase DB**를 공유합니다. 네트워크가 없어도 로컬 설정만으로 그대로 동작해요.

## 설치

1. 이 폴더의 `manifest.json`, `code.js`, `ui.html` 을 한 폴더에 내려받아요.
2. Figma 데스크톱 앱 → **Plugins → Development → Import plugin from manifest…**
3. `manifest.json` 을 선택 → **Plugins → Development → CreativeNamer** 로 실행.

## 사용

1. 이름을 바꿀 프레임(들)을 선택해요.
2. 구분 · 서비스 · 소재명을 지정해요.
3. **이대로 바꾸기** 를 누르면 선택한 프레임 이름이 한 번에 바뀝니다.

## 구성

| 파일 | 역할 |
| --- | --- |
| `manifest.json` | 플러그인 메타/네트워크 권한 |
| `code.js` | 메인 스레드 — 선택 감지, 프레임 이름 적용, 로컬 설정 저장(`clientStorage`) |
| `ui.html` | 패널 UI + 네이밍/미리보기/Supabase 동기화 로직 |

> 수정 요청은 마케팅팀 hansy에게 알려주세요.
