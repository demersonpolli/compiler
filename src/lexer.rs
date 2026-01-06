/// Tokens for the BASIC language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),
    // Operators
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
    OperatorPower,
    // Comparators
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
    // Punctuations
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    Newline,
    Eof,
    // Keywords
    //Activate,      // ACTIVATE statement          (6-4)
    //Arrival,       // ARRIVAL statement           (6-8)
    //Auto,          // AUTO command                (6-12)
    //Beep,          // BEEP statement              (6-13)
    //Bload,         // BLOAD statement             (6-14)
    //Bsave,         // BSAVE statement             (6-15)
    //Call,          // CALL statement              (6-16)
    //Calls,         // CALLS statement             (6-17)
    //Chain,         // CHAIN statement             (6-19)
    //ChDir,         // CHDIR statement             (6-22)
    //Circle,        // CIRCLE statement            (6-25)
    //Clear,         // CLEAR statement             (6-27)
    //Clip,          // CLIP statement              (6-30)
    //Close,         // CLOSE statement             (6-31)
    //Cls,           // CLS statement               (6-32)
    //Collision,     // COLLISION statement         (6-36)
    //Color,         // COLOR statement             (6-37/6-39)
    //Com,           // COM statement               (6-41)
    //Common,        // COMMON statement            (6-42)
    //Cont,          // CONT statement              (6-43)
    //Csrlin,        // CSRLIN variable             (6-46)
    //Data,          // DATA statement              (6-48)
    //DateS,         // DATE$ statement/variable    (6-49/6-50)
    //Deactivate,    // DEACTIVATE statement        (6-4)
    //DefFn,         // DEF FN statement            (6-51)
    //DefInt,        // DEFINT statement            (6-52)
    //DefDbl,        // DEFDBL statement            (6-52)
    //DefObject,     // DEF OBJECT statement        (6-53)
    //DefSeg,        // DEF SEG statement           (6-54)
    //DefSng,        // DEFSNG statement            (6-52)
    //DefStr,        // DEFSTR statement            (6-52)
    //DefUsr,        // DEF USR statement           (6-55)
    //Delete,        // DELETE command              (6-56)
    //Dim,           // DIM statement               (6-58)
    //DimObject,     // DIM Object statement        (6-57)
    //Draw,          // DRAW statement              (6-59)
    //Edit,          // EDIT command                (6-62)
    Else,          // IF ... THEN ... ELSE        (6-92)
    End,           // END statement               (6-63)
    //Environ,       // ENVIRON statement           (6-64)
    //Erase,         // ERASE statement             (6-69)
    //ErDev,         // ERDEV variable              (6-70)
    //ErDevS,        // ERDEV$ variable             (6-70)
    //Erl,           // ERL variable                (6-71)
    //Err,           // ERR variable                (6-71)
    //Error,         // ERROR statement             (6-72)
    //Field,         // FIELD statement             (6-75)
    //Files,         // FILES statement             (6-78)
    For,           // FOR ... NEXT statement      (6-81)
    //Get,           // GET statement               (6-85/6-86)
    //Gosub,         // GOSUB ... Return statement  (6-88)
    Goto,          // GOTO statement              (6-90)
    If,            // IF ... THEN ... ELSE        (6-92)
    Input,         // INPUT statement             (6-96)
    //InputN,        // INPUT# statement            (6-98)
    //IoCtl,         // IOCTL statement             (6-102)
    //Key,           // KEY statement               (6-104)
    //Keyn,          // KEY(n) statement            (6-107)
    //Kill,          // KILL statement              (6-109)
    Let,           // LET statement               (6-113)
    //Line,          // LINE statement              (6-114)
    //LineInput,     // LINE INPUT statement        (6-117)
    //LineInputN,    // LINE INPUT# statement       (6-118)
    //List,          // LIST command                (6-119)
    //Llist,         // LLIST command               (6-121)
    //Load,          // LOAD command                (6-122)
    //Locate,        // LOCATE statement            (6-124)
    //Lprint,        // LPRINT statement            (6-129)
    //LprintUsing,   // LPRINT USING statement      (6-129)
    //Lset,          // LSET statement              (6-130)
    //Merge,         // MERGE command               (6-131)
    //MidS,          // MID$ statement              (6-132)
    //MkDir,         // MKDIR statement             (6-134)
    //Name,          // NAME statement              (6-136)
    Next,          // FOR ... NEXT statement      (6-81)
    //New,           // NEW command                 (6-137)
    //Object,        // OBJECT statement            (6-139)
    //OnArrival,     // ON ARRIVAL statement        (6-143)
    //OnClip,        // ON CLIP statement           (6-145)
    //OnCollision,   // ON COLLISION statement      (6-147)
    //OnCom,         // ON COM statement            (6-150)
    //OnErrorGoto,   // ON ERROR GOTO statement     (6-152)
    //OnGosub,       // ON ... GOSUB statement      (6-153)
    //OnGoto,        // ON ... GOTO statement       (6-153)
    //OnKey,         // ON KEY statement            (6-154)
    //OnPlay,        // ON PLAY statement           (6-156)
    //OnStrig,       // ON STRIG statement          (6-157)
    //OnTimer,       // ON TIMER statement          (6-159)
    //Open,          // OPEN statement              (6-160)
    //OpenCom,       // OPENCOM statement           (6-162)
    //OptionBase,    // OPTION BASE statement       (6-164)
    //Out,           // OUT statement               (6-165)
    //Paint,         // PAINT statement             (6-166)
    //Palette,       // PALETTE statement           (6-169)
    //PaletteUsing,  // PALETTE USING statement     (6-171)
    //Play,          // PLAY statement              (6-174)
    //Poke,          // POKE statement              (6-181)
    //Preset,        // PRESET statement            (6-183)
    Print,         // PRINT statement             (6-184)
    //PrintUsing,    // PRINT USING statement       (6-187)
    //PrintN,        // PRINT# statement            (6-192)
    //PrintNUsing,   // PRINT# USING statement      (6-192)
    //Pset,          // PSET statement              (6-194)
    //Put,           // PUT statement               (6-196/197)
    //Randomize,     // RANDOMIZE statement         (6-199)
    //Read,          // READ statement              (6-201)
    Rem,           // REM statement               (6-203)
    //Renum,         // RENUM statement             (6-204)
    //Reset,         // RESET command               (6-205)
    //Restore,       // RESTORE statement           (6-206)
    //Resume,        // RESUME statement            (6-207)
    //Return,        // GOSUB ... RETURN statement  (6-88)
    //RmDir,         // RMDIR statement             (6-210)
    //Rset,          // RSET statement              (6-130)
    //Run,           // RUN command                 (6-212)
    //Save,          // SAVE command                (6-213)
    //Screen,        // SCREEN statement            (6-214)
    //Shell,         // SHELL statement             (6-218)
    //Sound,         // SOUND statement             (6-222)
    Step,          // FOR ... NEXT statement      (6-81)
    //StartObject,   // START OBJECT statement      (6-229)
    //Stop,          // STOP statement              (6-228)
    //StopObject,    // STOP OBJECT statement       (6-229)
    //Strig,         // STRIG statement             (6-231)
    //Swap,          // SWAP statement              (6-234)
    //System,        // SYSTEM command              (6-235)
    Then,          // IF ... THEN ... ELSE        (6-92)
    //TimeS,         // TIME$ variable              (6-239)
    //Timer,         // TIMER variable              (6-240)
    //TimerOff,      // TIMER OFF statement         (6-159)
    //TimerOn,       // TIMER ON statement          (6-159)
    //TimerStop,     // TIMER STOP statement        (6-159)
    To,            // FOR ... NEXT statement      (6-81)
    //Troff,         // TROFF statement             (6-241)
    //Tron,          // TRON statement              (6-241)
    //View,          // VIEW statement              (6-247)
    //ViewPrint,     // VIEW PRINT statement        (6-248)
    //Wait,          // WAIT statement              (6-249)
    //Wend,          // WHILE ... WEND statement    (6-250)
    //While,         // WHILE ... WEND statement    (6-250)
    //Width,         // WIDTH statement             (6-251)
    //Window,        // WINDOW statement            (6-253)
    //Write,         // WRITE statement             (6-255)
    //WriteN,        // WRITE# statement            (6-256)

    // Functions
    //AbsFunction,          // ABS() Function        (6-3)
    //ArrivalFunction,      // ARRIVAL() Function    (6-5)
    //AscFunction,          // ASC() Function        (6-10)
    //AtnFunction,          // ATN() Function        (6-11)
    //CdblFunction,         // CDBL() Function       (6-18)
    //ChrSFunction,         // CHR$() Function       (6-23)
    //CintFunction,         // CINT() Function       (6-24)
    //ClipFunction,         // CLIP() Function       (6-28)
    //CollisionFunction,    // COLLISION() Function  (6-33)
    //CosFunction,          // COS() Function        (6-44)
    //CsngFunction,         // CSNG() Function       (6-45)
    //CviFunction,          // CVI() Function        (6-47)
    //CvdFunction,          // CVD() Function        (6-47)
    //CvsFunction,          // CVS() Function        (6-47)
    //EnvironSFunction,     // ENVIRON$() Function   (6-66)
    //EofFunction,          // EOF() Function        (6-68)
    //ExpFunction,          // EXP() Function        (6-74)
    //FixFunction,          // FIX() Function        (6-80)
    //FreFunction,          // FRE() Function        (6-84)
    //HexSFunction,         // HEX$() Function       (6-91)
    //InkeySFunction,       // INKEY$() Function     (6-94)
    //InpFunction,          // INP() Function        (6-95)
    //InputSFunction,       // INPUT$() Function     (6-99)
    //InstrFunction,        // INSTR() Function      (6-100)
    //IntFunction,          // INT() Function        (6-101)
    //IoCtlSFunction,       // IOCTL$() Function     (6-103)
    //LeftSFunction,        // LEFT$() Function      (6-111)
    //LenFunction,          // LEN() Function        (6-112)
    //LocFunction,          // LOC() Function        (6-123)
    //LofFunction,          // LOF() Function        (6-126)
    //LogFunction,          // LOG() Function        (6-127)
    //LPosFunction,         // LPOS() Function       (6-128)
    //MidSFunction,         // MID$() Function       (6-133)
    //MkdSFunction,         // MKD$() Function       (6-135)
    //MkiSFunction,         // MKI$() Function       (6-135)
    //MksSFunction,         // MKS$() Function       (6-135)
    //ObjectFunction,       // OBJECT() Function     (6-138)
    //OctSFunction,         // OCT$() Function       (6-142)
    //PeekFunction,         // PEEK() Function       (6-173)
    //PlayFunction,         // PLAY() Function       (6-178)
    //PMapFunction,         // PMAP() Function       (6-179)
    //PointFunction,        // POINT() Function      (6-180)
    //PosFunction,          // POS() Function        (6-182)
    //RightFunction,        // RIGHT$() Function     (6-209)
    //RndFunction,          // RND() Function        (6-211)
    //ScreenFunction,       // SCREEN() Function     (6-216)
    //SgnFunction,          // SGN() Function        (6-217)
    //SinFunction,          // SIN() Function        (6-221)
    //SpaceSFunction,       // SPACE$() Function     (6-223)
    //SpcFunction,          // SPC() Function        (6-224)
    //SqrFunction,          // SQR() Function        (6-225)
    //StickFunction,        // STICK() Function      (6-226)
    //StrSFunction,         // STR$() Function       (6-230)
    //StrigFunction,        // STRIG() Function      (6-231)
    //StringSFunction,      // STRING$() Function    (6-233)
    //TabFunction,          // TAB() Function        (6-236)
    //TanFunction,          // TAN() Function        (6-237)
    //TimeSFunction,        // TIME$() Function      (6-238)
    //UsrFunction,          // USR() Function        (6-242)
    //ValFunction,          // VAL() Function        (6-243)
    //VarPtrFunction,       // VARPTR() Function     (6-244)
    //VarPtrSFunction,      // VARPTR$() Function    (6-245)
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        }
        else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            }
            else {
                break;
            }
        }
    }

    //fn read_number(&mut self) -> i64 {
    //    let mut num_string = String::new();
    //    while let Some(ch) = self.current_char() {
    //        if ch.is_ascii_digit() {
    //            num_string.push(ch);
    //            self.advance();
    //        }
    //        else {
    //            break;
    //        }
    //    }
    //    num_string.parse().unwrap()
    //}

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            }
            else {
                break;
            }
        }
        identifier
    }

    fn read_string(&mut self) -> String {
        self.advance(); // skip "
        let mut string = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                break;
            }
            string.push(ch);
            self.advance();
        }
        string
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('+') => {
                self.advance();
                Token::OperatorAdd
            }
            Some('-') => {
                self.advance();
                Token::OperatorSubtract
            }
            Some('*') => {
                self.advance();
                Token::OperatorMultiply
            }
            Some('/') => {
                self.advance();
                Token::OperatorDivide
            }
            Some('^') => {
                self.advance();
                Token::OperatorPower
            }
            Some('=') => {
                self.advance();
                Token::Equal
            }
            Some('<') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::LessOrEqual   // Using '[' to represent less or equal
                } else if self.current_char() == Some('>') {
                    self.advance();
                    Token::NotEqual      // Using '!' to represent different
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::GreaterOrEqual  // Using ']' to represent greater than
                } else {
                    Token::GreaterThan
                }
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some('"') => {
                let string = self.read_string();
                Token::String(string)
            }
            Some(ch) if ch.is_ascii_digit() || ch == '.' => {
                // Handling potentially floats or numbers starting with .
                if ch == '.' {
                    // Start of a float like .1
                    self.advance();
                    let mut num_string = String::from("0.");
                    while let Some(digit) = self.current_char() {
                        if digit.is_ascii_digit() {
                            num_string.push(digit);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    Token::Float(num_string.parse().unwrap_or(0.0))
                } else {
                    let mut num_string = String::new();
                    let mut is_float = false;
                    while let Some(digit) = self.current_char() {
                        if digit.is_ascii_digit() {
                            num_string.push(digit);
                            self.advance();
                        } else if digit == '.' && !is_float {
                            num_string.push(digit);
                            is_float = true;
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        Token::Float(num_string.parse().unwrap_or(0.0))
                    } else {
                        Token::Number(num_string.parse().unwrap_or(0))
                    }
                }
            }
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                // Match keywords (case-insensitive conversion)
                match identifier.to_uppercase().as_str() {
                    "ELSE" => Token::Else,
                    "END" => Token::End,
                    "FOR" => Token::For,
                    "GOTO" => Token::Goto,
                    "IF" => Token::If,
                    "INPUT" => Token::Input,
                    "LET" => Token::Let,
                    "NEXT" => Token::Next,
                    "PRINT" => Token::Print,
                    "THEN" => Token::Then,
                    "TO" => Token::To,
                    "REM" => {
                        // Skip until newline
                        while let Some(c) = self.current_char() {
                            if c == '\n' { break; }
                            self.advance();
                        }
                        Token::Rem
                    }
                    "STEP" => Token::Step,
                    _ => Token::Identifier(identifier),
                }
            }
            Some(ch) => {
                panic!("Unrecognized character: {}", ch);
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
