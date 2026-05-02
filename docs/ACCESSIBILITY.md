# Accessibility Guide

ProveIt is built on the principle that formal verification should be accessible to everyone. This document describes our accessibility commitments, implementation strategies, and guidelines for maintaining accessibility in contributions.

## Table of Contents

- [Our Commitment](#our-commitment)
- [Target Users](#target-users)
- [Compliance Standards](#compliance-standards)
- [Multi-Modal Design](#multi-modal-design)
- [Screen Reader Support](#screen-reader-support)
- [Keyboard Navigation](#keyboard-navigation)
- [Audio Feedback](#audio-feedback)
- [Haptic Feedback](#haptic-feedback)
- [Visual Design](#visual-design)
- [Cognitive Accessibility](#cognitive-accessibility)
- [Testing Accessibility](#testing-accessibility)
- [Reporting Issues](#reporting-issues)

## Our Commitment

ProveIt commits to:

1. **Accessibility as architecture**, not as an afterthought
2. **WCAG 2.1 Level AAA** compliance where applicable
3. **Multiple input methods** for every operation
4. **Multiple output modalities** for all information
5. **Consultation with disabled users** in design decisions
6. **Continuous improvement** based on real user feedback

We treat accessibility bugs with the same priority as security bugs.

## Target Users

ProveIt is designed for, among others:

### Blind and Low-Vision Users

- **Screen reader users** (NVDA, JAWS, VoiceOver, Orca)
- **Magnifier users** (ZoomText, built-in OS magnification)
- **High-contrast users** (custom themes, high-contrast modes)

### Neurodivergent Users

- **Autistic users** who benefit from predictable interfaces
- **ADHD users** who benefit from focused attention modes
- **Dyslexic users** who benefit from font and spacing options
- **Anxious users** who benefit from undo and confirmation

### Motor-Impaired Users

- **Limited mobility** users (one-handed, tremor)
- **Switch access** users (single switch, sip-and-puff)
- **Voice control** users (Dragon, Voice Control)
- **Keyboard-only** users (RSI, no mouse access)

### Cognitive-Impaired Users

- **Memory impairments** benefit from persistent state and history
- **Processing differences** benefit from configurable pacing
- **Reading difficulties** benefit from audio alternatives

### Multi-Disability Users

Many users have multiple conditions. ProveIt's design considers intersectional needs without forcing tradeoffs.

## Compliance Standards

### WCAG 2.1 Compliance

We target Level AAA where feasible:

| Principle | Level A | Level AA | Level AAA |
|-----------|---------|----------|-----------|
| Perceivable | Required | Required | Target |
| Operable | Required | Required | Target |
| Understandable | Required | Required | Target |
| Robust | Required | Required | Target |

### Specific Standards

- **WCAG 2.1** — primary web accessibility standard
- **EN 301 549** — European public sector accessibility
- **Section 508** — US federal accessibility
- **ADA** — Americans with Disabilities Act
- **PDF/UA** — for PDF documentation

### ARIA Patterns

We follow established ARIA patterns:

- [Application pattern](https://www.w3.org/WAI/ARIA/apg/patterns/) for the construction canvas
- [Tree pattern](https://www.w3.org/WAI/ARIA/apg/patterns/treeview/) for proof structure
- [Live region pattern](https://www.w3.org/TR/wai-aria-1.1/#live_region_roles) for verification feedback

## Multi-Modal Design

Every piece of information is available through multiple channels. Users choose which modalities to use.

### Information Channels

| Information Type | Visual | Audio | Haptic | Text |
|-----------------|--------|-------|--------|------|
| Object position | Coordinate display | Spatial pan | Edge buzz | "Point at (3, 4)" |
| Object type | Color/shape | Timbre | Pattern length | "Line" / "Circle" |
| Verification status | Color/icon | Tone quality | Vibration intensity | "Verified" / "Invalid" |
| Operation result | Animation | Confirmation tone | Pulse | "Step added" |
| Error condition | Red highlight | Alert tone | Long buzz | "Type mismatch" |

### Modality Independence

Each modality must convey complete information independently:

- **Visual-only would-be users** can rely entirely on visual presentation
- **Audio-only users** receive equivalent information via audio
- **Text-only users** can navigate the entire system with screen readers

This requires:

- No purely visual indicators (color alone, icons without labels)
- No purely audio indicators (sounds without text equivalents)
- All animations have static alternatives

## Screen Reader Support

### Supported Screen Readers

We test with:

- **NVDA** (Windows) — open source, primary test target
- **JAWS** (Windows) — commercial, widely used
- **VoiceOver** (macOS, iOS) — built-in Apple
- **TalkBack** (Android) — built-in Google
- **Orca** (Linux) — open source

### Announcement Strategy

Every interactive element has structured announcements:

```
[Element role]: [Element name], [Element state], [Element description]

Example:
"Construction canvas: 5 objects, focused on Point A at coordinates 3, 4.
Press Enter to select, arrow keys to navigate, ? for shortcuts."
```

### Live Regions

Verification updates use ARIA live regions:

```html
<!-- Status updates -->
<div role="status" aria-live="polite">
    Step 3 added: Identity introduction
</div>

<!-- Critical alerts -->
<div role="alert" aria-live="assertive">
    Verification failed at step 5: type mismatch
</div>
```

### Progressive Detail

Users can request more or less detail:

- **Brief mode**: Minimal announcements, only changes
- **Standard mode**: Default, balanced detail
- **Verbose mode**: Full mathematical context, hints
- **Custom mode**: User-configured detail levels

## Keyboard Navigation

### Universal Principles

Every action is accessible via keyboard:

1. **Tab order** follows logical document flow
2. **Focus indicators** are always visible
3. **Keyboard shortcuts** complement (not replace) tab navigation
4. **No keyboard traps** — every focused element can be left

### Standard Shortcuts

We follow platform conventions:

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New | Ctrl+N | Cmd+N |
| Open | Ctrl+O | Cmd+O |
| Save | Ctrl+S | Cmd+S |
| Undo | Ctrl+Z | Cmd+Z |
| Redo | Ctrl+Y / Ctrl+Shift+Z | Cmd+Shift+Z |
| Find | Ctrl+F | Cmd+F |
| Help | F1 | Cmd+? |

### ProveIt-Specific Shortcuts

Action-oriented shortcuts use mnemonic letters:

| Shortcut | Action |
|----------|--------|
| Ctrl+P | New **p**roof step |
| Ctrl+V | **V**erify current proof |
| Ctrl+R | Apply inference **r**ule |
| Ctrl+G | Insert **g**eometric primitive |
| Ctrl+T | Show **t**ype information |
| Ctrl+H | Show **h**istory |
| Alt+1-9 | Quick formal system selection |

### Customization

All shortcuts are user-configurable:

```toml
# ~/.config/proveit/shortcuts.toml
[shortcuts]
new_step = "Ctrl+P"
verify = "Ctrl+V"
help = "F1"

[chord]
proof_circle = "Ctrl+G,C"  # Chord shortcut
proof_point = "Ctrl+G,P"
```

### Switch Access

For users with severe motor limitations:

- All actions available through 2-switch scanning
- Configurable scan rate
- Audio confirmation of selections
- Cancel/back always one switch press away

## Audio Feedback

### Audio Cue Categories

ProveIt uses distinct audio categories that don't conflict:

| Category | Purpose | Characteristics |
|----------|---------|-----------------|
| Spatial | Object position | Stereo pan, frequency = position |
| Confirmation | Action success | Brief major chord |
| Error | Action failure | Brief minor chord, lower octave |
| Notification | Async update | Bell-like, brief |
| Ambient | Status background | Quiet, sustained |

### Spatial Audio Mapping

```
Object position → Audio properties:

x ∈ [-1, +1]  →  Stereo pan
y ∈ [0, 1]    →  Pitch (200Hz to 2000Hz, logarithmic)
z ∈ [0, 1]    →  Reverb depth (3D mode)
type          →  Timbre (sine, square, triangle, sawtooth)
state         →  Modulation (clean=valid, distorted=invalid)
```

### Operation Audio

Each operation has a distinct sound:

- **Point creation**: Brief tonic note
- **Line creation**: Sweep from start to end pitch
- **Circle creation**: Pulse at center frequency
- **Intersection**: Two-note interval
- **Verification success**: Major chord
- **Verification failure**: Tritone
- **Undo**: Reversed creation sound
- **Step navigation**: Stepped scale tones

### Configuration

Users can customize:

- **Master volume** with separate channels for cue types
- **Pitch range** for users with limited hearing range
- **Timbre selection** for personal preference
- **Mute by category** (e.g., disable ambient)
- **Audio-only mode** with enhanced verbosity

## Haptic Feedback

### Haptic Patterns

For devices with haptic support:

| Pattern | Purpose | Description |
|---------|---------|-------------|
| Tick | Object encounter | Single brief pulse |
| Bump | Boundary | Short double pulse |
| Pulse | Confirmation | Single longer pulse |
| Buzz | Error | Sustained vibration |
| Heartbeat | Connected | Two-pulse rhythm |

### Geometric Haptics

When navigating geometric objects:

- **Edge crossing** produces tick
- **Vertex** produces bump
- **Curve** produces continuous low vibration matching curvature
- **Intersection** produces double-tick
- **Selection boundary** produces pulse

### Configuration

- **Intensity scaling** for sensitivity preferences
- **Pattern customization** per event type
- **Disable for specific contexts** (e.g., during precision work)

## Visual Design

### Color Independence

No information is conveyed by color alone:

- **Verification states** use both color AND icons (✓ valid, ✗ invalid, ⟳ checking)
- **Object types** use both color AND shape variations
- **Errors** use both red AND distinctive iconography

### Color Schemes

Multiple color schemes available:

- **Default**: Balanced contrast for general use
- **High contrast**: Maximum contrast (WCAG AAA)
- **Dark mode**: Reduced eye strain in low light
- **Light mode**: Reduced reflection in bright environments
- **Deuteranopia**: Optimized for red-green color blindness
- **Tritanopia**: Optimized for blue-yellow color blindness
- **Monochrome**: For very low vision or print

### Typography

- **Default font**: Optimized for readability (e.g., Atkinson Hyperlegible)
- **Size scaling**: 100%–400% with maintained layout
- **Line spacing**: Adjustable (1.0–2.5×)
- **Letter spacing**: Adjustable for dyslexic users
- **Font choice**: User-selectable, including OpenDyslexic

### Visual Indicators

- **Focus rings**: 3px minimum, high contrast
- **Hover states**: Distinct from focus
- **Selection**: Both color and outline change
- **Active state**: Clearly differentiated from focus
- **Disabled state**: Visually distinct, but with sufficient contrast

## Cognitive Accessibility

### Predictable Interface

- **Consistent layout**: Same things in same places
- **Standard interactions**: Click, drag, type work as expected
- **Confirmation for destructive actions**: Always reversible or confirmed
- **Forgiving errors**: Easy to understand and recover from

### Reducing Cognitive Load

- **Progressive disclosure**: Advanced features hidden until needed
- **Templates**: Common patterns ready to use
- **Smart defaults**: Sensible starting points
- **Context-sensitive help**: Available without leaving current task

### Memory Support

- **Persistent state**: Resume where you left off
- **History**: Full record of all actions
- **Bookmarks**: Save points in long proofs
- **Notes**: Attach annotations to proof steps
- **Auto-save**: Continuous, non-intrusive

### Pacing

- **No time limits** on any operation
- **Pausable animations** with stop control
- **Adjustable verification speed** (real-time vs. on-demand)
- **No flashing content** above WCAG thresholds

### Language

- **Plain language** alongside technical terms
- **Glossary** linked from technical terms (see [GLOSSARY.md](GLOSSARY.md))
- **Examples** for every concept
- **Multiple complexity levels** (beginner, intermediate, advanced)

## Testing Accessibility

### Automated Testing

```bash
# Run accessibility test suite
cargo test --features=a11y-tests

# Specific accessibility checks
cargo test --test screen_reader_announcements
cargo test --test keyboard_navigation
cargo test --test color_contrast
```

### Manual Testing Checklist

For every change, verify:

#### Screen Reader

- [ ] All elements announced correctly
- [ ] Logical reading order
- [ ] Live regions update appropriately
- [ ] No redundant announcements
- [ ] Math content is comprehensible

#### Keyboard

- [ ] All actions reachable
- [ ] Logical tab order
- [ ] Visible focus indicators
- [ ] No keyboard traps
- [ ] Shortcuts work as documented

#### Visual

- [ ] Sufficient contrast (4.5:1 normal, 3:1 large text)
- [ ] No information by color alone
- [ ] Layout works at 200% zoom
- [ ] Layout works at 400% zoom (where applicable)
- [ ] All themes function correctly

#### Cognitive

- [ ] Errors are clear and actionable
- [ ] Forms are validated helpfully
- [ ] No surprising behavior
- [ ] Help is available

### User Testing

We conduct accessibility user testing:

- **Quarterly sessions** with disabled users
- **Compensation** for testers' time
- **Diverse representation** across disability types
- **Real-world tasks** rather than isolated features
- **Public reports** of findings (anonymized)

## Reporting Issues

### Accessibility Bug Reports

Use the [accessibility issue template](.github/ISSUE_TEMPLATE/accessibility_issue.md). Include:

1. **Your assistive technology** (screen reader, switch, etc.) and version
2. **Operating system** and version
3. **What you expected** to happen
4. **What actually happened**
5. **Impact**: blocking, frustrating, minor
6. **Workaround**: if any

### Priority

We treat accessibility issues with high priority:

- **Critical** (blocks essential use): Fixed within 1 week
- **High** (significant impact): Fixed within 1 month
- **Medium** (workaround exists): Fixed within 1 quarter
- **Low** (minor improvement): Tracked, fixed when possible

### Recognition

We thank accessibility issue reporters in:

- The relevant fix's commit message
- Release notes
- A11y contributors page (with permission)

## Resources

### Internal

- [Architecture: Accessibility Layer](../ARCHITECTURE.md#accessibility-layer)
- [Mathematical Glossary](GLOSSARY.md)
- [Tutorials](tutorials/)
- [Contributing Guidelines](../CONTRIBUTING.md)

### External

- [WCAG 2.1](https://www.w3.org/TR/WCAG21/)
- [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM](https://webaim.org/)
- [The A11y Project](https://www.a11yproject.com/)
- [Inclusive Components](https://inclusive-components.design/)

## Acknowledgments

This document is informed by:

- Disability community advocacy and feedback
- Accessibility research literature
- Platform vendor accessibility guidelines
- The lived experience of disabled developers and users

If you find anything in this document that fails to meet our commitments, please [report it](.github/ISSUE_TEMPLATE/accessibility_issue.md). We are committed to continuous improvement.
