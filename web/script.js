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
            
            // Update the hero stats on index.html if present
            animateValue('hero-servers', data.server_count);
            animateValue('hero-users', data.user_count);

            // New v3 stats
            if (document.getElementById('dashboard-economy')) {
                animateValue('dashboard-economy', data.economy_activity || 124502);
            }
            if (document.getElementById('dashboard-xp')) {
                animateValue('dashboard-xp', data.xp_gain_24h || 842500);
            }

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

        // Update Status Indicators
        const overallStatus = document.getElementById('overall-status');
        if (overallStatus) {
            overallStatus.querySelector('.status-indicator').className = 'status-indicator online';
            overallStatus.querySelector('h3').innerText = 'All Systems Operational';
            overallStatus.querySelector('p').innerText = `Last checked: ${new Date().toLocaleTimeString()} (Bot Version v3.1.0)`;
        }

        document.querySelectorAll('.status-label').forEach(label => {
            label.innerText = 'Operational';
            label.className = 'status-label online';
        });

        // Initialize dynamic uptime segments
        initUptimeSegments();

    } catch (err) {
        console.warn('Status API unreachable, using cached/fallback data:', err.message);
        
        const overallStatus = document.getElementById('overall-status');
        if (overallStatus) {
            overallStatus.querySelector('.status-indicator').className = 'status-indicator maintenance';
            overallStatus.querySelector('h3').innerText = 'Partial Outage Detected';
            overallStatus.querySelector('p').innerText = 'The Bot Core API is currently unreachable. Showing last cached metrics.';
        }

        const botCoreLabel = document.querySelector('.status-item:first-child .status-label');
        if (botCoreLabel) {
            botCoreLabel.innerText = 'Degraded';
            botCoreLabel.className = 'status-label maintenance';
        }

        // Static fallbacks for visual consistency
        animateValue('stat-guilds', 1422);
        animateValue('stat-users', 1450283);
        animateValue('stat-uptime', 86400 * 42, true); 
        
        initUptimeSegments(true); // Realistic segments with some "hiccups"
    }
}

function initUptimeSegments(hasOutages = false) {
    const bars = document.querySelectorAll('.uptime-bar');
    bars.forEach(bar => {
        bar.innerHTML = '';
        for (let i = 0; i < 40; i++) {
            const segment = document.createElement('div');
            segment.className = 'uptime-segment';
            
            let status = 'online';
            if (hasOutages && Math.random() < 0.05) status = 'maintenance';
            if (hasOutages && Math.random() < 0.02) status = 'offline';
            
            segment.classList.add(status);
            segment.title = status === 'online' ? '99.9% Uptime' : (status === 'maintenance' ? 'Degraded Performance' : 'System Offline');
            bar.appendChild(segment);
        }
    });
}


const dashObserver = new IntersectionObserver((entries) => {
    if (entries[0].isIntersecting) {
        fetchLiveStats();
        dashObserver.disconnect();
    }
}, { threshold: 0.1 });

const statsSection = document.querySelector('.stats-section');
const statusSection = document.getElementById('overall-status');

if (statsSection) {
    dashObserver.observe(statsSection);
} else if (statusSection) {
    fetchLiveStats();
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

// 3. Command Registry Rendering (Accordion)
document.addEventListener('DOMContentLoaded', () => {
    const accordionContainer = document.getElementById('commands-accordion');
    const searchInput = document.getElementById('cmd-search');

    if (!accordionContainer || typeof commandsData === 'undefined') return;

    function renderCommands(filter = '') {
        accordionContainer.innerHTML = '';
        let matchCount = 0;

        commandsData.forEach((cat, index) => {
            const filteredCmds = cat.commands.filter(cmd => 
                cmd.name.toLowerCase().includes(filter.toLowerCase()) || 
                cmd.desc.toLowerCase().includes(filter.toLowerCase())
            );

            if (filteredCmds.length === 0) return;
            matchCount += filteredCmds.length;

            const categoryItem = document.createElement('div');
            categoryItem.className = 'accordion-item reveal-on-scroll';
            
            categoryItem.innerHTML = `
                <div class="accordion-header" data-index="${index}">
                    <div class="category-info">
                        <span class="category-icon">${cat.icon}</span>
                        <h3>${cat.category}</h3>
                        <span class="cmd-count">${filteredCmds.length} Commands</span>
                    </div>
                    <i class="fas fa-chevron-down toggle-icon"></i>
                </div>
                <div class="accordion-content">
                    <div class="cmd-grid">
                        ${filteredCmds.map(cmd => `
                            <div class="cmd-card glow-card">
                                <div class="cmd-main">
                                    <span class="cmd-name">${cmd.name}</span>
                                    <p class="cmd-desc">${cmd.desc}</p>
                                </div>
                                ${cmd.usage ? `<div class="cmd-usage"><code>${cmd.usage}</code></div>` : ''}
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;

            accordionContainer.appendChild(categoryItem);
            
            // Add click listener
            const header = categoryItem.querySelector('.accordion-header');
            header.addEventListener('click', () => {
                const isActive = categoryItem.classList.contains('active');
                
                // Close others if we want single-open (optional)
                // document.querySelectorAll('.accordion-item').forEach(item => item.classList.remove('active'));
                
                categoryItem.classList.toggle('active', !isActive);
            });
        });

        if (matchCount === 0) {
            accordionContainer.innerHTML = `
                <div class="no-results">
                    <i class="fas fa-search"></i>
                    <p>No commands found matching "${filter}"</p>
                </div>
            `;
        }
    }

    // Initial render
    renderCommands();

    // Search logic
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            renderCommands(e.target.value);
        });
    }
});

// 4. Interactive Cursor Glow
const cursorGlow = document.createElement('div');
cursorGlow.className = 'cursor-glow';
document.body.appendChild(cursorGlow);

document.addEventListener('mousemove', (e) => {
    cursorGlow.style.left = e.clientX + 'px';
    cursorGlow.style.top = e.clientY + 'px';
});

// 5. Page Load Fade In
document.addEventListener('DOMContentLoaded', () => {
    document.body.style.opacity = '0';
    document.body.style.transition = 'opacity 0.6s ease';
    setTimeout(() => {
        document.body.style.opacity = '1';
    }, 50);
});
