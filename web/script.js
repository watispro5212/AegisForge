// ── Scroll-based nav opacity ──────────────────────────
const navbar = document.getElementById('navbar');
window.addEventListener('scroll', () => {
    navbar.classList.toggle('scrolled', window.scrollY > 40);
});

// ── Particle system ───────────────────────────────────
const particleContainer = document.getElementById('particles');
const PARTICLE_COUNT = 40;

for (let i = 0; i < PARTICLE_COUNT; i++) {
    const p = document.createElement('div');
    p.className = 'particle';
    p.style.left = `${Math.random() * 100}%`;
    p.style.bottom = `${Math.random() * -20}%`;
    const size = Math.random() * 3 + 1;
    p.style.width = `${size}px`;
    p.style.height = `${size}px`;
    const dur = Math.random() * 15 + 10;
    p.style.animationDuration = `${dur}s`;
    p.style.animationDelay = `${Math.random() * dur}s`;
    particleContainer.appendChild(p);
}

// ── Scroll reveal for feature cards ──────────────────
const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry, i) => {
        if (entry.isIntersecting) {
            setTimeout(() => {
                entry.target.classList.add('revealed');
            }, i * 100);
            observer.unobserve(entry.target);
        }
    });
}, { threshold: 0.1 });

document.querySelectorAll('[data-reveal]').forEach(el => observer.observe(el));

// ── Command tabs ──────────────────────────────────────
document.querySelectorAll('.cmd-tab').forEach(tab => {
    tab.addEventListener('click', () => {
        const target = tab.dataset.tab;

        document.querySelectorAll('.cmd-tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.cmd-panel').forEach(p => p.classList.remove('active'));

        tab.classList.add('active');
        const panel = document.getElementById(`tab-${target}`);
        if (panel) {
            panel.classList.add('active');
            panel.style.animation = 'none';
            panel.offsetHeight; // reflow
            panel.style.animation = 'fadeSlideIn 0.3s ease-out forwards';
        }
    });
});

// Inject keyframe for tab transition
const style = document.createElement('style');
style.textContent = `
    @keyframes fadeSlideIn {
        from { opacity: 0; transform: translateY(8px); }
        to   { opacity: 1; transform: translateY(0); }
    }
`;
document.head.appendChild(style);

// ── Smooth scroll for anchor links ───────────────────
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', (e) => {
        e.preventDefault();
        const target = document.querySelector(anchor.getAttribute('href'));
        if (target) {
            target.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }
    });
});
