use gpui::{img, svg, Hsla, Rems, Transformation};

use crate::{prelude::*, Size};
use rui_macros::DerivePathStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use strum::{EnumIter, EnumString, IntoStaticStr};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    /// 10px
    Indicator,
    /// 12px
    XSmall,
    /// 14px
    Small,
    #[default]
    /// 16px
    Medium,
    /// 24px
    Large,
    /// 48px
    XLarge,
    Custom(Rems),
}

impl From<Size> for IconSize {
    fn from(size: Size) -> Self {
        match size {
            Size::XSmall => IconSize::XSmall,
            Size::Small => IconSize::Small,
            Size::Medium => IconSize::Medium,
            Size::Large => IconSize::Large,
            Size::Custom(size) => IconSize::Custom(size),
        }
    }
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::Indicator => rems_from_px(10.),
            IconSize::XSmall => rems_from_px(12.),
            IconSize::Small => rems_from_px(14.),
            IconSize::Medium => rems_from_px(16.),
            IconSize::Large => rems_from_px(24.),
            IconSize::XLarge => rems_from_px(48.),
            IconSize::Custom(size) => size,
        }
    }
}

/// The source of an icon.
enum IconSource {
    /// An SVG embedded in the Zed binary.
    Svg(SharedString),
    /// An image file located at the specified path.
    ///
    /// Currently our SVG renderer is missing support for the following features:
    /// 1. Loading SVGs from external files.
    /// 2. Rendering polychrome SVGs.
    ///
    /// In order to support icon themes, we render the icons as images instead.
    Image(Arc<Path>),
}

impl IconSource {
    fn from_path(path: impl Into<SharedString>) -> Self {
        let path = path.into();
        if path.starts_with("icons/file_icons") {
            Self::Svg(path)
        } else {
            Self::Image(Arc::from(PathBuf::from(path.as_ref())))
        }
    }
}

#[derive(IntoElement)]
pub struct Icon {
    source: IconSource,
    color: Option<Hsla>,
    size: Rems,
    transformation: Transformation,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Self {
            source: IconSource::Svg(icon.path().into()),
            color: None,
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn from_path(path: impl Into<SharedString>) -> Self {
        Self {
            source: IconSource::from_path(path),
            color: None,
            size: IconSize::default().rems(),
            transformation: Transformation::default(),
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size.rems();
        self
    }

    /// Sets a custom size for the icon, in [`Rems`].
    ///
    /// Not to be exposed outside of the `ui` crate.
    pub(crate) fn custom_size(mut self, size: Rems) -> Self {
        self.size = size;
        self
    }

    pub fn transform(mut self, transformation: Transformation) -> Self {
        self.transformation = transformation;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or(window.text_style().color);

        match self.source {
            IconSource::Svg(path) => svg()
                .with_transformation(self.transformation)
                .size(self.size)
                .flex_none()
                .path(path)
                .text_color(color)
                .into_any_element(),
            IconSource::Image(path) => img(path)
                .size(self.size)
                .flex_none()
                .text_color(color)
                .into_any_element(),
        }
    }
}

#[derive(Debug, IntoElement, Copy, Clone, EnumIter, EnumString, IntoStaticStr, DerivePathStr)]
#[strum(serialize_all = "snake_case")]
#[path_str(prefix = "icons", suffix = ".svg")]
pub enum IconName {
    Ai,
    AiAnthropic,
    AiAnthropicHosted,
    AiDeepSeek,
    AiGoogle,
    AiLmStudio,
    AiOllama,
    AiOpenAi,
    AiZed,
    ArrowCircle,
    ArrowDown,
    ArrowDownFromLine,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowUpFromLine,
    ArrowUpRight,
    AtSign,
    AudioOff,
    AudioOn,
    Backspace,
    Bell,
    BellDot,
    BellOff,
    BellRing,
    Blocks,
    Bolt,
    Book,
    BookCopy,
    BookPlus,
    CaseSensitive,
    Check,
    ChevronDown,
    /// This chevron indicates a popover menu.
    ChevronDownSmall,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronUpDown,
    Circle,
    Close,
    Code,
    Command,
    Context,
    Control,
    Copilot,
    CopilotDisabled,
    CopilotError,
    CopilotInit,
    Copy,
    CountdownTimer,
    CursorIBeam,
    Dash,
    DatabaseZap,
    Delete,
    Diff,
    Disconnected,
    Download,
    Ellipsis,
    EllipsisVertical,
    Envelope,
    Eraser,
    Escape,
    ExpandVertical,
    Exit,
    ExternalLink,
    Eye,
    EyeOff,
    File,
    FileCode,
    FileDoc,
    FileDiff,
    FileGeneric,
    FileGit,
    FileLock,
    FileRust,
    FileSearch,
    FileText,
    FileToml,
    FileTree,
    Filter,
    Folder,
    FolderOpen,
    FolderX,
    Font,
    FontSize,
    FontWeight,
    GenericClose,
    GenericMaximize,
    GenericMinimize,
    GenericRestore,
    Github,
    Globe,
    GitBranch,
    Hash,
    HistoryRerun,
    Indicator,
    Info,
    InlayHint,
    Keyboard,
    Library,
    LineHeight,
    Link,
    ListTree,
    ListX,
    Loading,
    LoadingHalf,
    LockOutlined,
    MagnifyingGlass,
    MailOpen,
    Maximize,
    Menu,
    MessageBubbles,
    MessageCircle,
    Mic,
    MicMute,
    Microscope,
    Minimize,
    Option,
    PageDown,
    PageUp,
    PanelLeft,
    PanelRight,
    Pencil,
    Person,
    PersonCircle,
    PhoneIncoming,
    Pin,
    Play,
    Plus,
    PocketKnife,
    Public,
    PullRequest,
    Quote,
    RefreshTitle,
    Regex,
    ReplNeutral,
    Replace,
    ReplaceAll,
    ReplaceNext,
    ReplyArrowRight,
    Rerun,
    Return,
    Reveal,
    RotateCcw,
    RotateCw,
    Route,
    Save,
    Screen,
    SearchCode,
    SearchSelection,
    SelectAll,
    Server,
    Settings,
    SettingsAlt,
    Shift,
    Slash,
    SlashSquare,
    Sliders,
    SlidersVertical,
    Snip,
    Space,
    Sparkle,
    SparkleAlt,
    SparkleFilled,
    Spinner,
    Split,
    SquareDot,
    SquareMinus,
    SquarePlus,
    Star,
    StarFilled,
    Stop,
    Strikethrough,
    Supermaven,
    SupermavenDisabled,
    SupermavenError,
    SupermavenInit,
    SwatchBook,
    Tab,
    Terminal,
    TextSnippet,
    ThumbsUp,
    ThumbsDown,
    Trash,
    TrashAlt,
    Triangle,
    TriangleRight,
    Undo,
    Unpin,
    Update,
    UserGroup,
    Wand,
    Warning,
    WholeWord,
    X,
    XCircle,
    ZedAssistant,
    ZedAssistant2,
    ZedAssistantFilled,
    ZedPredict,
    ZedPredictDisabled,
    ZedXCopilot,
}

impl RenderOnce for IconName {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::new(icon)
    }
}
