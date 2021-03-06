use makepad_render::*;

// Shader code itself

fn shader() -> ShaderGen {Quad::def_quad_shader().compose(shader!{"
    fn pixel() -> vec4 {
        let ratio = vec2(
            mix(w / h, 1.0, float(w <= h)),
            mix(1.0, h / w, float(w <= h))
        );
        let p0 = vec3((2.0 * pos - 1.0) *  ratio, 2.0); 
        let v = vec3(0.0, 0.0, -1.0);
        let m = identity() * rotation(vec3(1.0, 1.0, 1.0), time);
        p0 = (m * vec4(p0, 1.0)).xyz;
        v = (m * vec4(v, 0.0)).xyz;
        let t = march_ray(p0, v);
        if t.x < T_MAX {
            let p = p0 + t.x * v;
            let n = estimate_normal(p);

            let c = vec4(0.0);
            if t.y == 0.0 || t.y == 1.0 {
                c += pick!(#856F37);
            }
            if t.y == 2.0 {
                c += pick!(#11EF00);
            }
            if t.y == 3.0 {
                c += pick!(#FD8C00); 
            }
            if t.y == 4.0 {
                c += pick!(#FF0000);
            }
            
            let ld = normalize(vec3(0.0, 0.0, 1.0));
            let ls = normalize(vec3(0.0, 0.0, 1.0));
            let v = normalize(p0);
            let r = slide!(1.0)*2.0 * dot(n, ls) * n - ls;
            
            let ia = 0.2;
            let id = 0.3 * max(0.0, dot(ld, n));
            let is = 0.5 * pow(max(0.0, dot(v, r)), slide!(0.8851563)*2.0);
            let i = ia + id + is;
            
            return i * c; 
        } else {
            return vec4(0.0);  
        }
    }

    fn sdf(p: vec3) -> vec2 {
        return displace(p, union(
            //cube(p),
            intersection(cube(p), sphere(p)),
            union(union(cylinder_x(p), cylinder_y(p)), cylinder_z(p))
        ));
    }

    const EPSILON: float = 1E-3;
    const T_MAX: float = 10.0;
    
    fn identity() -> mat4 {
        return mat4(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0
        );
    }
    
    fn rotation(axis: vec3, angle: float) -> mat4 {
        let u = normalize(axis);
        let s = sin(angle);
        let c = cos(angle);
        return mat4(
            c + u.x * u.x * (1.0 - c),
            u.y * u.x * (1.0 - c) + u.z * s,
            u.z * u.x * (1.0 - c) - u.y * s,
            0.0,
            u.x * u.y * (1.0 - c) - u.z * s,
            c + u.y * u.y * (1.0 - c),
            u.z * u.y * (1.0 - c) + u.x * s,
            0.0,
            u.x * u.z * (1.0 - c) + u.y * s,
            u.y * u.z * (1.0 - c) - u.x * s,
            c + u.z * u.z * (1.0 - c),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0
        );
    }
    
    fn cube(p: vec3) -> vec2 {
        let q = abs(p) - 0.4;
        return vec2(min(max(q.x, max(q.y, q.z)), 0.0) + length(max(q, 0.0)), 0.0);
    }
    
    fn cylinder_x(p: vec3) -> vec2 {
        let d = abs(vec2(length(p.yz), p.x)) - vec2(0.25, 0.75);
        return vec2(min(max(d.x, d.y), 0.0) + length(max(d, 0.0)), 2.0);
    }
    
    fn cylinder_y(p: vec3) -> vec2 {
        let d = abs(vec2(length(p.xz), p.y)) - vec2(0.25, 0.75);
        return vec2(min(max(d.x, d.y), 0.0) + length(max(d, 0.0)), 3.0);
    }
    
    fn cylinder_z(p: vec3) -> vec2 {
        let d = abs(vec2(length(p.xy), p.z)) - vec2(0.25, 0.75);
        return vec2(min(max(d.x, d.y), 0.0) + length(max(d, 0.0)), 4.0);
    }
    
    fn displace(p: vec3, d: vec2) -> vec2 {
        return vec2(0.05 * sin(10.0* p.x) * sin(10.0 * p.y) * sin(10.0* p.z) + d.x, d.y);
    }
    
    fn difference(d1: vec2, d2: vec2) -> vec2 {
        return vec2(max(d1.x, -d2.x), mix(d1.y, d2.y, float(d1.x < -d2.x)));
    }
    
    fn intersection(d1: vec2, d2: vec2) -> vec2 {
        return vec2(max(d1.x, d2.x), mix(d1.y, d2.y, float(d1.x < d2.x)));
    }
    
    fn sphere(p: vec3) -> vec2 {
        return vec2(length(p) - 0.5, 1.0);
    }
    
    fn union(d1: vec2, d2: vec2) -> vec2 {
        return vec2(min(d1.x, d2.x), mix(d2.y, d1.y, float(d1.x < d2.x)));
    }
    
    fn estimate_normal(p: vec3) -> vec3 {
        return normalize(vec3(
            sdf(vec3(p.x + EPSILON, p.y, p.z)).x - sdf(vec3(p.x - EPSILON, p.y, p.z)).x,
            sdf(vec3(p.x, p.y + EPSILON, p.z)).x - sdf(vec3(p.x, p.y - EPSILON, p.z)).x,
            sdf(vec3(p.x, p.y, p.z + EPSILON)).x - sdf(vec3(p.x, p.y, p.z - EPSILON)).x
        ));
    }
    
    fn march_ray(p0: vec3, v: vec3) -> vec2 {
        let t = 0.0;
        for i from 0 to 100 {
            let d = sdf(p0 + t * v);
            if d.x <= EPSILON {
                return vec2(t, d.y);
            }
            t += d.x * 0.5; 
            if t >= T_MAX {
                break;
            }
        }
        return vec2(T_MAX, 0.0);
    }

"})}

// Makepad UI structure to render shader

#[derive(Clone)]
pub struct ShaderView {
    quad: Quad,
    area: Area,
    animator: Animator,
    finger_hover: Vec2,
    finger_move: Vec2,
    finger_down: f32,
    time: f32
}

impl ShaderView {
    pub fn bg() -> ShaderId {uid!()}
    pub fn finger_hover() -> Vec2Id {uid!()}
    pub fn finger_move() -> Vec2Id {uid!()}
    pub fn finger_down() -> FloatId {uid!()}
    pub fn time() -> FloatId {uid!()}
    pub fn new(cx: &mut Cx) -> Self {
        
        Self::bg().set(cx, shader().compose(shader!{"
            instance finger_hover: ShaderView::finger_hover();
            instance finger_move: ShaderView::finger_move();
            instance finger_down: ShaderView::finger_down();
            instance time: ShaderView::time();
        "}));
         
        Self {
            quad: Quad::new(cx),
            area: Area::default(),
            animator: Animator::default(),
            finger_hover: Vec2::default(),
            finger_move: Vec2::default(),
            finger_down: 0.0,
            time: 0.0
        }
    }
    
    pub fn handle_shader_view(&mut self, cx: &mut Cx, event: &mut Event) {
        match event.hits(cx, self.area, HitOpt::default()) {
            Event::Frame(_ae)=>{
                self.time += 1.0/60.0;
                self.area.write_float(cx, Self::time(), self.time);
                cx.next_frame(self.area);
            },
            Event::FingerMove(fm) => {
                self.finger_move = fm.rel;
                self.area.write_vec2(cx, Self::finger_move(), self.finger_move);
            },
            Event::FingerHover(fm) => {
                self.finger_hover = fm.rel;
                self.area.write_vec2(cx, Self::finger_hover(), self.finger_hover);
            },
            Event::FingerDown(_fd) => {
                self.finger_down = 1.0;
                self.area.write_float(cx, Self::finger_down(), self.finger_down);
            },
            Event::FingerUp(_fu) => {
                self.finger_down = 0.0;
                self.area.write_float(cx, Self::finger_down(), self.finger_down);
            },
            _ => ()
        }
    }
    
    pub fn draw_shader_view(&mut self, cx: &mut Cx) {
        self.quad.shader = Self::bg().get(cx);
        let k = self.quad.draw_quad_abs(cx, cx.get_turtle_rect());
        k.push_vec2(cx, self.finger_hover);
        k.push_vec2(cx, self.finger_move);
        k.push_float(cx, self.finger_down);
        k.push_float(cx, self.time);
        self.area = cx.update_area_refs(self.area, k.into());
        cx.next_frame(self.area);
    }
}

