use std::collections::HashMap;
use std::fmt::Debug;

use crate::layout::RenderNode;

#[derive(Debug, Default)]
pub struct RenderCtx {
    pub depth: usize,
}

pub trait Glyph: Debug {
    fn required_args(&self) -> usize {
        0
    }

    fn has_optional(&self) -> bool {
        false
    }

    fn render(&self, args: &[RenderNode], _opts: &[RenderNode], _ctx: &mut RenderCtx)
        -> RenderNode;
}

pub struct SymbolRegistry {
    map: HashMap<String, Box<dyn Glyph>>,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: impl Into<String>, glyph: impl Glyph + 'static) {
        self.map.insert(name.into(), Box::new(glyph));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Glyph> {
        self.map.get(name).map(|g| g.as_ref())
    }
}

#[derive(Debug)]
pub struct UnicodeGlyph(pub char);

impl Glyph for UnicodeGlyph {
    fn render(
        &self,
        _args: &[RenderNode],
        _opts: &[RenderNode],
        _ctx: &mut RenderCtx,
    ) -> RenderNode {
        RenderNode::from_char(self.0)
    }
}

#[derive(Debug)]
pub struct TextGlyph(pub &'static str);

impl Glyph for TextGlyph {
    fn render(
        &self,
        _args: &[RenderNode],
        _opts: &[RenderNode],
        _ctx: &mut RenderCtx,
    ) -> RenderNode {
        RenderNode::from_str(self.0)
    }
}

#[derive(Debug)]
pub struct FracGlyph;

impl Glyph for FracGlyph {
    fn required_args(&self) -> usize {
        2
    }

    fn render(&self, args: &[RenderNode], _opts: &[RenderNode], ctx: &mut RenderCtx) -> RenderNode {
        let pad = if ctx.depth == 0 { 1 } else { 0 };
        RenderNode::vstack(&args[0], &args[1], '─', pad)
    }
}

#[derive(Debug)]
pub struct SqrtGlyph;

impl Glyph for SqrtGlyph {
    fn required_args(&self) -> usize {
        1
    }

    fn has_optional(&self) -> bool {
        true
    }

    fn render(&self, args: &[RenderNode], opts: &[RenderNode], _ctx: &mut RenderCtx) -> RenderNode {
        let radicand = RenderNode::sqrt_inner(&args[0]);
        if let Some(root) = opts.first() {
            let w = root.width + radicand.width;
            let h = root.height.max(radicand.height);
            let mut data = vec![' '; w * h];

            root.blit_into(&mut data, w, 0, 0);
            radicand.blit_into(&mut data, w, root.width, 0);

            RenderNode {
                width: w,
                height: h,
                baseline: radicand.baseline,
                data,
            }
        } else {
            radicand
        }
    }
}

#[derive(Debug)]
pub struct AbsGlyph;

impl Glyph for AbsGlyph {
    fn required_args(&self) -> usize {
        1
    }

    fn render(
        &self,
        args: &[RenderNode],
        _opts: &[RenderNode],
        _ctx: &mut RenderCtx,
    ) -> RenderNode {
        RenderNode::abs(&args[0])
    }
}
