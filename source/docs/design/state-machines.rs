
struct CrashSafeLog {
  persistent: LogSpec,
  ephemeral: LogSpec, // two "copies" of the same state machine (state)
  log: Map<nat, LogSpec>, // two "copies" of the same state machine (state)
}

impl CrashSafeLog {
    pub fn init(&self) {
        self.persistent.init()
        self.ephemeral == self.persistent
    }

    pub fn ephemeral_move(old, new)
    {
      // -> &&& persistent == persistent'

      &&& old.persistent == new.persistent
      // &&& old.ephemeral.next(new.epehemeral)
      &&& (old.ephemeral, new.ephemeral).next()

      &&& (old.log[i], new.log[i]).next()
      &&& action.log[i].next()

      // action.log[i]: (LogSpec, LogSpec)

      // disallow 
      // &&&
      // |||
      // &&&
    }
    // more appealing than
    pub fn ephemeral_move(old, new)
    {
      and([
        old.persistent == new.persistent,
        old.ephemeral(new),
      ])
    }

    // imperative
    // pub fn ephemeral_move(old, new) {
    //   new == Self {
    //     old.ephemeral.next(),
    //     ...old
    //   }
    // }

    new == Self {
        // TODO left here with Jon
        ephemeral: x :| action(old.ephemeral, x).next()
    }

    #[and] pub fn ephemeral_move(&old, &new)
    // affects ephemeral
    {
      old.persistent == new.persistent,
      old.ephemeral(new),
    }

    init predicate()
    {
        && persistent.init()
        && ephemeral == persistent
    }
}

// can we support both imperative and predicate for twostate

