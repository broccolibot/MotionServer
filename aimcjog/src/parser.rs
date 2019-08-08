use libaimc::AIMCMessage;
use std::fmt;

pub const HELP_LINES: &[&str] = &[
    "AIMC Jogger help:",
    "\tset target <float> ",
    "\tset enable <bool>",
    "\tset home <int>",
    "\tset kp <float>",
    "\tset ki <float>",
    "\tset kd <float>",
    "\tset polarity <bool>",
    "\tget",
];

pub enum ActionParseError<'a> {
    Unrecognized(&'a str),
    MissingArg(&'static str),
    At(&'a str),
}

impl fmt::Display for ActionParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionParseError::Unrecognized(level) => write!(f, "Did not recognize '{}'.", level),
            ActionParseError::At(text) => write!(f, "Error parsing at '{}'.", text),
            ActionParseError::MissingArg(arg) => write!(f, "Missing '{}' argument.", arg),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Write(AIMCMessage),
    Help,
    Read,
}

fn parse_arg<'a, T: std::str::FromStr>(
    args: &mut Iterator<Item = &'a str>,
    missingerr: &'static str,
) -> Result<T, ActionParseError<'a>> {
    match args.next() {
        Some(text) => text.parse::<T>().map_err(|_| ActionParseError::At(text)),
        None => Err(ActionParseError::MissingArg(missingerr)),
    }
}

fn aimcmessage_from_str<'a>(
    args: &mut Iterator<Item = &'a str>,
) -> Result<AIMCMessage, ActionParseError<'a>> {
    match args
        .next()
        .ok_or(ActionParseError::MissingArg("parameter"))?
    {
        "target" | "t" => Ok(AIMCMessage::SetTarget(parse_arg(args, "target")?)),
        "enable" | "e" => Ok(AIMCMessage::Enable(parse_arg(args, "enabled")?)),
        "home" | "h" => Ok(AIMCMessage::Home(parse_arg(args, "speed")?)),
        "kp" | "p" => Ok(AIMCMessage::SetKp(parse_arg(args, "kP")?)),
        "ki" | "i" => Ok(AIMCMessage::SetKi(parse_arg(args, "kI")?)),
        "kd" | "d" => Ok(AIMCMessage::SetKd(parse_arg(args, "kD")?)),
        "limit" | "lim" => Ok(AIMCMessage::LimitPwm(parse_arg(args, "PWM")?)),
        "limittargetmin" | "ltmi" => Ok(AIMCMessage::LimitTargetMin(parse_arg(args, "target")?)),
        "limittargetmax" | "ltma" => Ok(AIMCMessage::LimitTargetMax(parse_arg(args, "target")?)),
        "polarity" | "pl" => Ok(AIMCMessage::EncoderPolarity(parse_arg(args, "polarity")?)),
        other => Err(ActionParseError::Unrecognized(other)),
    }
}

impl Action {
    pub fn from_commandline<'a>(
        args: &mut Iterator<Item = &'a str>,
    ) -> Result<Self, ActionParseError<'a>> {
        //match args.next().ok_or(ActionParseError::MissingArg("command"))? {
        match args.next().unwrap_or("g") {
            "set" | "write" | "s" => Ok(Action::Write(aimcmessage_from_str(args)?)),
            "help" => Ok(Action::Help),
            "get" | "read" | "g" => Ok(Action::Read),
            other => Err(ActionParseError::Unrecognized(other)),
        }
    }
}
