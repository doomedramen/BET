# 26 — Multi-Channel Behaviour [OPTIONAL]

> How the system behaves across different devices, platforms, and entry points.
> Include this when the system is accessed via multiple channels with different constraints.
>
> **Dependencies**: [06-flow-state.md](06-flow-state.md), [16-interaction-behaviour.md](16-interaction-behaviour.md)

---

## Supported Channels

| Channel | Description | Constraints |
|---|---|---|
| _[e.g., Desktop web]_ | _[Full browser on desktop/laptop]_ | _[No constraints — full feature set]_ |
| _[e.g., Mobile web]_ | _[Mobile browser on phone/tablet]_ | _[Smaller viewport, touch input, limited multitasking]_ |
| _[e.g., Native iOS app]_ | _[iOS application]_ | _[Platform guidelines, push notifications, biometric auth]_ |
| _[e.g., Native Android app]_ | _[Android application]_ | _[Platform guidelines, push notifications, back button]_ |
| _[e.g., Email deeplink]_ | _[User clicks link in email]_ | _[Must handle expired sessions, specific entry state]_ |
| _[e.g., SMS link]_ | _[User clicks link in SMS]_ | _[Very short URL, mobile-first landing]_ |
| _[e.g., API consumer]_ | _[Third-party integration]_ | _[No UI — data only]_ |

---

## Behavioural Differences

_Where the system behaves differently per channel._

| Behaviour | Desktop web | Mobile web | Native app | Notes |
|---|---|---|---|---|
| _[e.g., File upload]_ | _[Drag & drop + file picker]_ | _[File picker + camera]_ | _[Camera + gallery + file picker]_ | — |
| _[e.g., Navigation]_ | _[Sidebar + breadcrumbs]_ | _[Bottom nav + back button]_ | _[Tab bar + swipe gestures]_ | — |
| _[e.g., Notifications]_ | _[In-app + email]_ | _[In-app + email]_ | _[Push + in-app + email]_ | — |
| _[e.g., Session timeout]_ | _[Redirect to login]_ | _[Redirect to login]_ | _[Biometric re-auth]_ | — |

---

## Cross-Channel Continuity

_How users can switch between channels without losing progress._

- **State sync**: _[Real-time / on next load / manual sync]_
- **Data persistence**: _[Server-side (always available) / local (per device)]_
- **Flow resumption**: _[Can a user start on mobile and continue on desktop?]_
- **Conflicts**: _[What happens if data is modified on two channels simultaneously?]_

---

## Channel-Specific Entry Points

_How different channels enter the system and what state they start in._

| Entry point | Channel | Landing state | Pre-conditions |
|---|---|---|---|
| _[e.g., Marketing email CTA]_ | _[Email → web]_ | _[STATE-001 with pre-filled email]_ | _[Valid tracking token]_ |
| _[e.g., Push notification]_ | _[Native app]_ | _[Quote results]_ | _[Valid session, quote exists]_ |
| _[e.g., QR code on letter]_ | _[Mobile web]_ | _[STATE-001 with pre-filled reference]_ | _[Valid reference number]_ |

---

## Browser / Platform Support

| Browser / Platform | Minimum version | Notes |
|---|---|---|
| _[e.g., Chrome]_ | _[last 2 major versions]_ | — |
| _[e.g., Safari]_ | _[last 2 major versions]_ | _[Test date picker specifically]_ |
| _[e.g., Firefox]_ | _[last 2 major versions]_ | — |
| _[e.g., Edge]_ | _[last 2 major versions]_ | — |
| _[e.g., iOS Safari]_ | _[iOS 15+]_ | _[Primary mobile browser]_ |
| _[e.g., Chrome Android]_ | _[last 2 major versions]_ | — |
