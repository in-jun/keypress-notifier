# Keypress Notifier

`keypress-notifier`는 리눅스에서 동작하는 입력 이벤트 감지 및 벨 소리 알림 프로젝트입니다.

## 소개

`keypress-notifier`는 Rust 언어로 개발되었으며, 키패드, 마우스 등의 입력 이벤트를 감지하고, 벨 소리로 사용자에게 알립니다.

## 사용법

1. Rust를 설치합니다.

    ```bash
    # Rust를 설치하는 명령어 (https://www.rust-lang.org/ko/tools/install 참고)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. 프로젝트를 클론하고 실행합니다.

    ```bash
    git clone https://github.com/in-jun/keypress-notifier.git
    cd keypress-notifier
    cargo build --release
    sudo ./target/release/keypress-notifier
    ```

3. 키를 누르고, 클릭하고, 움직여보세요!

## 주요 기능

-   리눅스 전용: `keypress-notifier`는 리눅스 환경에서만 작동합니다.
-   감각적인 입력: 키 이벤트에 소리로 반응하여 사용자 경험을 높입니다.

---

**Rust로 작성된 `keypress-notifier`로 색다른 입력 경험을 즐겨보세요! 🎶🐧**
