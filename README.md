# 두런허브 데스크탑

두런허브 공식 데스크탑 앱 (Tauri 기반)

## 지원 플랫폼
- Windows (x64)
- macOS (Apple Silicon / Intel)
- Linux

## 기능
- 🌐 두런허브 전체 기능
- 🔔 새 메시지 OS 알림
- 🖥️ 트레이 상주 (닫아도 백그라운드 유지)
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

## 릴리즈
GitHub에 `v1.0.0` 태그를 push하면 자동으로 Windows/macOS/Linux 설치파일이 빌드됩니다.
