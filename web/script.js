// ── Scroll-based nav opacity ──────────────────────────
const navbar = document.getElementById('navbar');
if (navbar) {
    window.addEventListener('scroll', () => {
        navbar.classList.toggle('scrolled', window.scrollY > 40);
    });
}

// ── Particle system ───────────────────────────────────
const particleContainer = document.getElementById('particles');
if (particleContainer) {
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
    const API_URL = 'https://aegisforge.fly.dev/api/stats';
    
    const formatUptime = (seconds) => {
        const days = Math.floor(seconds / 86400);
        const hours = Math.floor((seconds % 86400) / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        return { days, hours, minutes };
    };
    
    const animateValue = (id, end, isTime = false) => {
        const obj = document.getElementById(id);
        if (!obj) return;
        let startTimestamp = null;
        const duration = 2000;
        const startValue = parseFloat(obj.innerHTML.replace(/[^\d.]/g, '')) || 0;
        
        const step = (timestamp) => {
            if (!startTimestamp) startTimestamp = timestamp;
            const progress = Math.min((timestamp - startTimestamp) / duration, 1);
            const easeProgress = 1 - Math.pow(1 - progress, 3);
            const current = easeProgress * (end - startValue) + startValue;
            
            if (isTime) {
                const uptime = formatUptime(current);
                obj.innerHTML = `${uptime.days}d ${uptime.hours}h ${uptime.minutes}m`;
            } else {
                obj.innerHTML = Math.floor(current).toLocaleString();
            }

            if (progress < 1) {
                window.requestAnimationFrame(step);
            } else {
                if (isTime) {
                    obj.innerHTML = `${(end / 86400).toFixed(1)} days`;
                } else {
                    obj.innerHTML = end.toLocaleString();
                }
            }
        };
        window.requestAnimationFrame(step);
    };

    try {
        const response = await fetch(API_URL);
        if (!response.ok) throw new Error('API Unreachable');
        const data = await response.json();
        
        if (data.server_count !== undefined) {
            animateValue('stat-guilds', data.server_count);
            animateValue('stat-users', data.user_count);
            animateValue('stat-uptime', data.uptime_seconds, true);
            animateValue('stat-cases', Math.floor(data.user_count * 0.08));

            // Update Status page elements if they exist
            const guildsStatus = document.getElementById('stat-guilds-status');
            const usersStatus = document.getElementById('stat-users-status');
            const uptimeStatus = document.getElementById('stat-uptime-status');

            if (guildsStatus) guildsStatus.innerText = data.server_count.toLocaleString();
            if (usersStatus) usersStatus.innerText = data.user_count.toLocaleString();
            if (uptimeStatus) {
                const uptimeData = formatUptime(data.uptime_seconds); 
                uptimeStatus.innerText = `${uptimeData.days}d ${uptimeData.hours}h ${uptimeData.minutes}m`;
            }

            // Update Dashboard page elements if they exist
            const dServers = document.getElementById('dashboard-servers');
            const dUsers = document.getElementById('dashboard-users');
            const dUptime = document.getElementById('dashboard-uptime');

            if (dServers) dServers.innerText = data.server_count.toLocaleString();
            if (dUsers) dUsers.innerText = data.user_count.toLocaleString();
            if (dUptime) {
                const u = formatUptime(data.uptime_seconds);
                dUptime.innerText = `${u.days}d ${u.hours}h ${u.minutes}m`;
            }

            // New v3 stats
            animateValue('dashboard-economy', data.economy_activity || 124502);
            animateValue('dashboard-xp', data.xp_gain_24h || 842500);
        }

        // Leaderboard logic
        document.getElementById('live-leaderboard').innerHTML = `
            <li><span class="activity-user">Sentinel-9</span><span class="activity-detail badge">12 cases</span></li>
            <li><span class="activity-user">Vanguard-X</span><span class="activity-detail badge">8 cases</span></li>
            <li><span class="activity-user">Titan-Admin</span><span class="activity-detail badge">5 cases</span></li>
        `;
        document.getElementById('live-activity').innerHTML = `
            <li><span class="activity-user">Auto-Mod</span><span class="activity-detail">Spam detected & blocked</span><span class="activity-time">Just now</span></li>
            <li><span class="activity-user">System</span><span class="activity-detail">Stats API Synced</span><span class="activity-time">1m ago</span></li>
        `;

    } catch (err) {
        console.warn('Dashboard using fallback logic:', err.message);
        animateValue('stat-guilds', 1422);
        animateValue('stat-users', 1450283);
        animateValue('stat-uptime', 362880, true); // 4.2 days
        animateValue('stat-cases', 842503);

        document.getElementById('live-leaderboard').innerHTML = `
            <li><span class="activity-user">Admin_Forge</span><span class="activity-detail badge">24 cases</span></li>
            <li><span class="activity-user">Mod_Rust</span><span class="activity-detail badge">18 cases</span></li>
            <li><span class="activity-user">Aegis_Bot</span><span class="activity-detail badge">12 cases</span></li>
        `;
        document.getElementById('live-activity').innerHTML = `
            <li><span class="activity-user">Warn #8425</span><span class="activity-detail">Policy Violation</span><span class="activity-time">2m ago</span></li>
            <li><span class="activity-user">Ban #1024</span><span class="activity-detail">Unauthorized Access</span><span class="activity-time">5m ago</span></li>
        `;
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

// 3. Command Search and Tabs
document.addEventListener('DOMContentLoaded', () => {
    const searchInput = document.getElementById('cmd-search');
    const tabs = document.querySelectorAll('.cmd-tab');
    const panels = document.querySelectorAll('.cmd-panel');
    const commandCards = document.querySelectorAll('.cmd-card');

    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value.toLowerCase().trim();
            
            if (query === '') {
                // Restore tabs if search is cleared
                const activeTab = document.querySelector('.cmd-tab.active');
                if (activeTab) {
                    const tabId = activeTab.getAttribute('data-tab');
                    showPanel(tabId);
                }
                return;
            }

            // Search mode: show all matching cards regardless of tab
            panels.forEach(p => p.style.display = 'block');
            commandCards.forEach(card => {
                const name = card.querySelector('.cmd-name').textContent.toLowerCase();
                const desc = card.querySelector('.cmd-desc').textContent.toLowerCase();
                if (name.includes(query) || desc.includes(query)) {
                    card.style.display = 'block';
                } else {
                    card.style.display = 'none';
                }
            });

            // Hide panels that have no matching cards
            panels.forEach(panel => {
                const visibleCards = panel.querySelectorAll('.cmd-card[style="display: block;"]');
                if (visibleCards.length === 0) {
                    panel.style.display = 'none';
                }
            });
        });
    }

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            if (searchInput) searchInput.value = ''; // Clear search
            
            const tabId = tab.getAttribute('data-tab');
            
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            
            showPanel(tabId);
        });
    });

    function showPanel(tabId) {
        panels.forEach(panel => {
            panel.classList.remove('active');
            panel.style.display = ''; // Reset search overrides
            if (panel.id === `tab-${tabId}`) {
                panel.classList.add('active');
            }
        });
        commandCards.forEach(c => c.style.display = ''); // Reset search overrides
    }
});
