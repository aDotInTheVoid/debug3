use crate::{Formatter, Write};

pub(crate) struct PadAdapter<'buf, 'state> {
    pub(crate) buf: &'buf mut (dyn Write + 'buf),
    pub(crate) state: &'state mut PadAdapterState,
}

pub(crate) struct PadAdapterState {
    pub(crate) on_newline: bool,
}

impl Default for PadAdapterState {
    fn default() -> Self {
        PadAdapterState { on_newline: true }
    }
}

impl<'buf, 'state> PadAdapter<'buf, 'state> {
    pub(crate) fn wrap<'slot, 'fmt: 'buf + 'slot>(
        fmt: &'fmt mut Formatter<'_>,
        slot: &'slot mut Option<Self>,
        state: &'state mut PadAdapterState,
    ) -> Formatter<'slot> {
        fmt.wrap_buf(move |buf| slot.insert(PadAdapter { buf, state }))
    }
}

impl Write for PadAdapter<'_, '_> {
    fn write_str(&mut self, mut s: &str) {
        while !s.is_empty() {
            if self.state.on_newline {
                self.buf.write_str("    ");
            }

            let split = match s.find('\n') {
                Some(pos) => {
                    self.state.on_newline = true;
                    pos + 1
                }
                None => {
                    self.state.on_newline = false;
                    s.len()
                }
            };
            self.buf.write_str(&s[..split]);
            s = &s[split..];
        }
    }
}
