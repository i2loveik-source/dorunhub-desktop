# 두런허브 데스크탑

두런허브 공식 데스크탑 앱 (Tauri 기반)

## 지원 플랫폼
- Windows (x64)
- macOS (Apple Silicon / Intel)
- Linux

## 핵심 기능
- 🌐 두런허브 전체 기능 (웹뷰)
- 📱 모바일과 유사한 **컴팩트 기본 창 크기** (430x900)
- 🔔 새 메시지/담벼락 새 글 **OS 알림**
- 🖥️ 트레이 상주 (닫아도 백그라운드 유지)
- 🧭 트레이 메뉴에서 앱 열기/컴팩트 리사이즈/종료
- 🚀 부팅 시 자동 시작 옵션

## 개발 환경 설정

```bash
# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 의존성 설치
npm install

# 개발 모드 실행
npm run dev

# 빌드
npm run build
```

## 배포 산출물
- macOS: `.app`, `.dmg`
- Windows: `.msi`, `.exe`(설정에 따라)

## 릴리즈
GitHub에 태그를 push하면 자동으로 Windows/macOS/Linux 설치파일 빌드 가능.
예: `v1.1.0`
