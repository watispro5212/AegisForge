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

// ── Live Dashboard Data Fetching ────────────────────
async function fetchLiveStats() {
    try {
        const response = await fetch('/api/stats');
        if (!response.ok) throw new Error('Network error');
        const data = await response.json();
        
        if (data.stats) {
            const animateValue = (id, end) => {
                const obj = document.getElementById(id);
                if (!obj) return;
                let startTimestamp = null;
                const duration = 1500;
                const step = (timestamp) => {
                    if (!startTimestamp) startTimestamp = timestamp;
                    const progress = Math.min((timestamp - startTimestamp) / duration, 1);
                    obj.innerHTML = Math.floor(progress * end);
                    if (progress < 1) {
                        window.requestAnimationFrame(step);
                    } else {
                        obj.innerHTML = end;
                    }
                };
                window.requestAnimationFrame(step);
            };

            animateValue('stat-guilds', data.stats.guilds);
            animateValue('stat-cases', data.stats.cases);
            animateValue('stat-warnings', data.stats.warnings);
            animateValue('stat-reminders', data.stats.reminders);
        }

        const leaderboardEl = document.getElementById('live-leaderboard');
        if (data.leaderboard && data.leaderboard.length > 0) {
            leaderboardEl.innerHTML = data.leaderboard.map(item => `
                <li>
                    <span class="activity-user">User ${item.user_id.substring(0, 8)}...</span>
                    <span class="activity-detail badge">${item.infraction_count} warnings</span>
                </li>
            `).join('');
        } else {
            leaderboardEl.innerHTML = `<li><span class="activity-detail">No data available</span></li>`;
        }

        const activityEl = document.getElementById('live-activity');
        if (data.recentActivity && data.recentActivity.length > 0) {
            activityEl.innerHTML = data.recentActivity.map(item => `
                <li>
                    <span class="activity-user">Warn #${item.id}</span>
                    <span class="activity-detail">${item.reason}</span>
                    <span class="activity-time">${new Date(item.created_at).toLocaleDateString()}</span>
                </li>
            `).join('');
        } else {
            activityEl.innerHTML = `<li><span class="activity-detail">No recent activity</span></li>`;
        }
    } catch (error) {
        console.error("Failed to load live stats:", error);
        document.getElementById('live-leaderboard').innerHTML = `<li><span class="activity-detail">Stats Offline</span></li>`;
        document.getElementById('live-activity').innerHTML = `<li><span class="activity-detail">Stats Offline</span></li>`;
    }
}

const dashObserver = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
        fetchLiveStats();
        dashObserver.disconnect();
    }
}, { threshold: 0.1 });

const dashboardSection = document.getElementById('dashboard');
if (dashboardSection) {
    dashObserver.observe(dashboardSection);
}

/* ─── MASSIVE UI OVERHAUL SCRIPT ADDITIONS ───────────────────── */

// 1. Intersection Observer for Scroll Reveals
const revealElements = document.querySelectorAll('.reveal-on-scroll, .feature-card, .stack-card');
const revealOptions = {
    threshold: 0.15,
    rootMargin: "0px 0px -50px 0px"
};

const revealOnScroll = new IntersectionObserver(function(entries, observer) {
    entries.forEach(entry => {
        if (!entry.isIntersecting) return;
        entry.target.classList.add('visible', 'revealed');
        observer.unobserve(entry.target);
    });
}, revealOptions);

revealElements.forEach(el => revealOnScroll.observe(el));

// 2. 3D Tilt Effect for Cards
const tiltCards = document.querySelectorAll('.tilt-card, .feature-card, .dashboard-card, .stack-card, .pricing-card');

tiltCards.forEach(card => {
    // Ensure styles are set correctly
    card.style.transformStyle = 'preserve-3d';
    card.style.transition = 'transform 0.1s ease-out';
    
    // Add child translation for pop-out effect
    Array.from(card.children).forEach(child => {
        if (!child.style.transform && !child.classList.contains('hero-glow') && !child.classList.contains('cta-glow')) {
            child.style.transform = 'translateZ(20px)';
        }
    });

    card.addEventListener('mousemove', e => {
        const rect = card.getBoundingClientRect();
        const x = e.clientX - rect.left; // x position within the element
        const y = e.clientY - rect.top;  // y position within the element
        
        const centerX = rect.width / 2;
        const centerY = rect.height / 2;
        
        // Calculate rotation based on cursor position
        const rotateX = ((y - centerY) / centerY) * -10; // Max 10 deg
        const rotateY = ((x - centerX) / centerX) * 10;
        
        card.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(1.02, 1.02, 1.02)`;
    });

    card.addEventListener('mouseleave', () => {
        card.style.transform = `perspective(1000px) rotateX(0deg) rotateY(0deg) scale3d(1, 1, 1)`;
        // Smooth snap back
        card.style.transition = 'transform 0.5s cubic-bezier(0.2, 0.8, 0.2, 1)';
        setTimeout(() => {
            card.style.transition = 'transform 0.1s ease-out';
        }, 500);
    });
});
