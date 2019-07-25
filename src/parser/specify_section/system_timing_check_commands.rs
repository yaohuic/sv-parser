use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum SystemTimingCheck {
    SetupTimingCheck(Box<SetupTimingCheck>),
    HoldTimingCheck(Box<HoldTimingCheck>),
    SetupholdTimingCheck(Box<SetupholdTimingCheck>),
    RecoveryTimingCheck(Box<RecoveryTimingCheck>),
    RemovalTimingCheck(Box<RemovalTimingCheck>),
    RecremTimingCheck(Box<RecremTimingCheck>),
    SkewTimingCheck(Box<SkewTimingCheck>),
    TimeskewTimingCheck(Box<TimeskewTimingCheck>),
    FullskewTimingCheck(Box<FullskewTimingCheck>),
    PeriodTimingCheck(Box<PeriodTimingCheck>),
    WidthTimingCheck(Box<WidthTimingCheck>),
    NochargeTimingCheck(Box<NochargeTimingCheck>),
}

#[derive(Clone, Debug, Node)]
pub struct SetupTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            DataEvent,
            Symbol,
            ReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct HoldTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct SetupholdTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<TimestampCondition>,
                    Option<(
                        Symbol,
                        Option<TimecheckCondition>,
                        Option<(
                            Symbol,
                            Option<DelayedReference>,
                            Option<(Symbol, Option<DelayedData>)>,
                        )>,
                    )>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct RecoveryTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct RemovalTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct RecremTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<TimestampCondition>,
                    Option<(
                        Symbol,
                        Option<TimecheckCondition>,
                        Option<(
                            Symbol,
                            Option<DelayedReference>,
                            Option<(Symbol, Option<DelayedData>)>,
                        )>,
                    )>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct SkewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct TimeskewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<EventBasedFlag>,
                    Option<(Symbol, Option<RemainActiveFlag>)>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct FullskewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<EventBasedFlag>,
                    Option<(Symbol, Option<RemainActiveFlag>)>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct PeriodTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ControlledReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct WidthTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ControlledReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            Threshold,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct NochargeTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            StartEdgeOffset,
            Symbol,
            EndEdgeOffset,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn system_timing_check(s: Span) -> IResult<Span, SystemTimingCheck> {
    alt((
        map(setup_timing_check, |x| {
            SystemTimingCheck::SetupTimingCheck(Box::new(x))
        }),
        map(hold_timing_check, |x| {
            SystemTimingCheck::HoldTimingCheck(Box::new(x))
        }),
        map(setuphold_timing_check, |x| {
            SystemTimingCheck::SetupholdTimingCheck(Box::new(x))
        }),
        map(recovery_timing_check, |x| {
            SystemTimingCheck::RecoveryTimingCheck(Box::new(x))
        }),
        map(removal_timing_check, |x| {
            SystemTimingCheck::RemovalTimingCheck(Box::new(x))
        }),
        map(recrem_timing_check, |x| {
            SystemTimingCheck::RecremTimingCheck(Box::new(x))
        }),
        map(skew_timing_check, |x| {
            SystemTimingCheck::SkewTimingCheck(Box::new(x))
        }),
        map(timeskew_timing_check, |x| {
            SystemTimingCheck::TimeskewTimingCheck(Box::new(x))
        }),
        map(fullskew_timing_check, |x| {
            SystemTimingCheck::FullskewTimingCheck(Box::new(x))
        }),
        map(period_timing_check, |x| {
            SystemTimingCheck::PeriodTimingCheck(Box::new(x))
        }),
        map(width_timing_check, |x| {
            SystemTimingCheck::WidthTimingCheck(Box::new(x))
        }),
        map(nocharge_timing_check, |x| {
            SystemTimingCheck::NochargeTimingCheck(Box::new(x))
        }),
    ))(s)
}

#[parser]
pub(crate) fn setup_timing_check(s: Span) -> IResult<Span, SetupTimingCheck> {
    let (s, a) = keyword("$setup")(s)?;
    let (s, b) = paren(tuple((
        data_event,
        symbol(","),
        referecne_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SetupTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn hold_timing_check(s: Span) -> IResult<Span, HoldTimingCheck> {
    let (s, a) = keyword("$setup")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, HoldTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn setuphold_timing_check(s: Span) -> IResult<Span, SetupholdTimingCheck> {
    let (s, a) = keyword("$setuphold")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(timestamp_condition),
                opt(triple(
                    symbol(","),
                    opt(timecheck_condition),
                    opt(triple(
                        symbol(","),
                        opt(delayed_reference),
                        opt(pair(symbol(","), opt(delayed_data))),
                    )),
                )),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SetupholdTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn recovery_timing_check(s: Span) -> IResult<Span, RecoveryTimingCheck> {
    let (s, a) = keyword("$recovery")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RecoveryTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn removal_timing_check(s: Span) -> IResult<Span, RemovalTimingCheck> {
    let (s, a) = keyword("$removal")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RemovalTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn recrem_timing_check(s: Span) -> IResult<Span, RecremTimingCheck> {
    let (s, a) = keyword("$recrem")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(timestamp_condition),
                opt(triple(
                    symbol(","),
                    opt(timecheck_condition),
                    opt(triple(
                        symbol(","),
                        opt(delayed_reference),
                        opt(pair(symbol(","), opt(delayed_data))),
                    )),
                )),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RecremTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn skew_timing_check(s: Span) -> IResult<Span, SkewTimingCheck> {
    let (s, a) = keyword("$skew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SkewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn timeskew_timing_check(s: Span) -> IResult<Span, TimeskewTimingCheck> {
    let (s, a) = keyword("$timeskew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(event_based_flag),
                opt(pair(symbol(","), opt(remain_active_flag))),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, TimeskewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn fullskew_timing_check(s: Span) -> IResult<Span, FullskewTimingCheck> {
    let (s, a) = keyword("$fullskew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(event_based_flag),
                opt(pair(symbol(","), opt(remain_active_flag))),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, FullskewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn period_timing_check(s: Span) -> IResult<Span, PeriodTimingCheck> {
    let (s, a) = keyword("$period")(s)?;
    let (s, b) = paren(tuple((
        controlled_referecne_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, PeriodTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn width_timing_check(s: Span) -> IResult<Span, WidthTimingCheck> {
    let (s, a) = keyword("$width")(s)?;
    let (s, b) = paren(tuple((
        controlled_referecne_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        threshold,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, WidthTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn nocharge_timing_check(s: Span) -> IResult<Span, NochargeTimingCheck> {
    let (s, a) = keyword("$nocharge")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        start_edge_offset,
        symbol(","),
        end_edge_offset,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, NochargeTimingCheck { nodes: (a, b, c) }))
}