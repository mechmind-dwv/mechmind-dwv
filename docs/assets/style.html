<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MechMind-dwv | 🤖 Ingeniería Robótica</title>
    <link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🤖</text></svg>">
    <style>
        /* Variables CSS */
        :root {
            --mechmind-primary: #00D4FF;
            --mechmind-secondary: #FF00FF;
            --mechmind-accent: #00FF88;
            --mechmind-bg: #0D1117;
            --mechmind-surface: #161B22;
            --mechmind-text: #E6EDF3;
            --mechmind-border: #30363D;
            --font-mono: 'Fira Code', 'Courier New', monospace;
        }

        /* Reset y base */
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            background: var(--mechmind-bg);
            color: var(--mechmind-text);
            font-family: var(--font-mono);
            line-height: 1.6;
            overflow-x: hidden;
        }

        /* Terminal principal */
        .cyber-terminal {
            border: 2px solid var(--mechmind-primary);
            border-radius: 8px;
            margin: 2rem auto;
            max-width: 900px;
            background: var(--mechmind-surface);
            box-shadow: 0 0 20px rgba(0, 212, 255, 0.3);
            position: relative;
            z-index: 10;
        }

        .terminal-header {
            background: var(--mechmind-border);
            padding: 0.5rem 1rem;
            border-bottom: 1px solid var(--mechmind-primary);
            display: flex;
            align-items: center;
        }

        .terminal-title {
            color: var(--mechmind-primary);
            font-size: 0.9rem;
            font-weight: bold;
        }

        .terminal-content {
            padding: 2rem;
        }

        .cyber-title {
            font-size: 2.5rem;
            color: var(--mechmind-primary);
            text-align: center;
            margin-bottom: 0.5rem;
            text-shadow: 0 0 10px var(--mechmind-primary);
        }

        .cyber-subtitle {
            text-align: center;
            color: var(--mechmind-accent);
            margin-bottom: 2rem;
            font-style: italic;
        }

        /* Efecto typewriter */
        .terminal-output {
            background: #000;
            padding: 1.5rem;
            border-radius: 4px;
            border: 1px solid var(--mechmind-border);
            margin: 2rem 0;
            min-height: 200px;
            font-size: 0.9rem;
        }

        .type {
            display: block;
            opacity: 0;
            animation: fadeIn 0.5s forwards;
        }

        @keyframes fadeIn {
            to { opacity: 1; }
        }

        /* Botones */
        .button-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-top: 2rem;
        }

        .cyber-button {
            background: transparent;
            color: var(--mechmind-primary);
            border: 2px solid var(--mechmind-primary);
            padding: 1rem 2rem;
            text-decoration: none;
            text-align: center;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
            font-family: var(--font-mono);
            font-weight: bold;
        }

        .cyber-button:hover {
            color: var(--mechmind-bg);
            background: var(--mechmind-primary);
            text-shadow: none;
            box-shadow: 0 0 20px var(--mechmind-primary);
            transform: translateY(-2px);
        }

        .cyber-button.secondary {
            border-color: var(--mechmind-secondary);
            color: var(--mechmind-secondary);
        }

        .cyber-button.secondary:hover {
            background: var(--mechmind-secondary);
            box-shadow: 0 0 20px var(--mechmind-secondary);
        }

        .cyber-button.accent {
            border-color: var(--mechmind-accent);
            color: var(--mechmind-accent);
        }

        .cyber-button.accent:hover {
            background: var(--mechmind-accent);
            box-shadow: 0 0 20px var(--mechmind-accent);
        }

        /* Secciones */
        .tech-section, .projects-section {
            padding: 4rem 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }

        .tech-section h2, .projects-section h2 {
            text-align: center;
            color: var(--mechmind-primary);
            margin-bottom: 3rem;
            font-size: 2rem;
        }

        .tech-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 2rem;
        }

        .tech-item {
            background: var(--mechmind-surface);
            padding: 2rem;
            border-radius: 8px;
            border: 1px solid var(--mechmind-border);
            text-align: center;
            transition: all 0.3s ease;
        }

        .tech-item:hover {
            border-color: var(--mechmind-primary);
            box-shadow: 0 0 15px rgba(0, 212, 255, 0.2);
            transform: translateY(-5px);
        }

        .tech-icon {
            font-size: 3rem;
            display: block;
            margin-bottom: 1rem;
        }

        .tech-name {
            font-size: 1.2rem;
            color: var(--mechmind-primary);
            font-weight: bold;
        }

        .project-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 2rem;
        }

        .project-card {
            background: var(--mechmind-surface);
            padding: 2rem;
            border-radius: 8px;
            border: 1px solid var(--mechmind-border);
            transition: all 0.3s ease;
        }

        .project-card:hover {
            border-color: var(--mechmind-accent);
            box-shadow: 0 0 20px rgba(0, 255, 136, 0.2);
            transform: translateY(-5px);
        }

        .project-card h3 {
            color: var(--mechmind-accent);
            margin-bottom: 1rem;
        }

        .project-link {
            color: var(--mechmind-primary);
            text-decoration: none;
            font-weight: bold;
            display: inline-block;
            margin-top: 1rem;
        }

        .project-link:hover {
            text-shadow: 0 0 10px var(--mechmind-primary);
        }

        /* Footer */
        .cyber-footer {
            text-align: center;
            padding: 3rem 2rem;
            border-top: 1px solid var(--mechmind-border);
            margin-top: 4rem;
            color: var(--mechmind-text);
            font-style: italic;
        }

        /* Canvas de partículas */
        #particles {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            z-index: 1;
        }

        /* Responsive */
        @media (max-width: 768px) {
            .cyber-terminal {
                margin: 1rem;
                border-radius: 4px;
            }
            
            .terminal-content {
                padding: 1rem;
            }
            
            .cyber-title {
                font-size: 2rem;
            }
            
            .button-grid {
                grid-template-columns: 1fr;
            }
            
            .tech-grid, .project-grid {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    <div class="cyber-terminal">
        <div class="terminal-header">
            <span class="terminal-title">MechMind-dwv@github:~$</span>
        </div>
        
        <div class="terminal-content">
            <h1 class="cyber-title">🤖 MechMind-dwv</h1>
            <p class="cyber-subtitle">Con ❤️ hacia lo que nos une: la evolución conjunta</p>
            
            <pre id="typewriter" class="terminal-output">
<span class="type" data-delay="0">$ git status</span>
<span class="type" data-delay="1000">On branch main</span>
<span class="type" data-delay="2000">$ cargo build --release</span>
<span class="type" data-delay="3000">🚀 Compilando MechBot v3.0...</span>
<span class="type" data-delay="4000">✅ Sistema robótico inicializado</span>
<span class="type" data-delay="5000">🤖 IA generativa: ACTIVA</span>
<span class="type" data-delay="6000">❤️ Conexión humano-máquina establecida</span>
<span class="type" data-delay="7000">🧑‍🚀 Preparados para la evolución</span>
            </pre>
            
            <div class="button-grid">
                <a href="https://github.com/mechmind-dwv" class="cyber-button primary">
                    🤖 Ver Repositorios
                </a>
                <a href="#tech-stack" class="cyber-button secondary">
                    ❤️ Tech Stack
                </a>
                <a href="#projects" class="cyber-button accent">
                    🧑‍🚀 Proyectos
                </a>
            </div>
        </div>
    </div>
    
    <section id="tech-stack" class="tech-section">
        <h2>❤️ Stack Tecnológico</h2>
        <div class="tech-grid">
            <div class="tech-item">
                <span class="tech-icon">🤖</span>
                <span class="tech-name">Rust</span>
            </div>
            <div class="tech-item">
                <span class="tech-icon">🧑‍🚀</span>
                <span class="tech-name">Python</span>
            </div>
            <div class="tech-item">
                <span class="tech-icon">🤖</span>
                <span class="tech-name">ROS2</span>
            </div>
            <div class="tech-item">
                <span class="tech-icon">❤️</span>
                <span class="tech-name">AI/ML</span>
            </div>
        </div>
    </section>
    
    <section id="projects" class="projects-section">
        <h2>🧑‍🚀 Proyectos Destacados</h2>
        <div class="project-grid">
            <div class="project-card">
                <h3>🤖 MechBot 3.0</h3>
                <p>Sistema robótico autónomo con ❤️ integrada</p>
                <a href="https://github.com/mechmind-dwv/mechbot-3x" class="project-link">Ver Proyecto →</a>
            </div>
            <div class="project-card">
                <h3>🧑‍🚀 Neural Nexus</h3>
                <p>Plataforma de IA distribuida para evolución conjunta</p>
                <a href="https://github.com/mechmind-dwv/neural-nexus" class="project-link">Ver Proyecto →</a>
            </div>
        </div>
    </section>
    
    <footer class="cyber-footer">
        <p>"🤖 + ❤️ + 🧑‍🚀 = Evolución infinita"</p>
        <p>— Un lienzo en blanco, con amor a lo que nos une</p>
        <p>— La capacidad de evolucionar juntos</p>
    </footer>
    
    <canvas id="particles"></canvas>
    
    <script>
        // Efecto typewriter mejorado
        function initTypewriter() {
            const elements = document.querySelectorAll('.type');
            
            elements.forEach((el, index) => {
                const text = el.textContent;
                const delay = parseInt(el.getAttribute('data-delay')) || index * 1000;
                
                el.textContent = '';
                el.style.opacity = '0';
                
                setTimeout(() => {
                    el.style.opacity = '1';
                    let i = 0;
                    const typing = setInterval(() => {
                        if (i < text.length) {
                            el.textContent += text[i];
                            i++;
                        } else {
                            clearInterval(typing);
                        }
                    }, 50);
                }, delay);
            });
        }

        // Sistema de partículas
        function initParticles() {
            const canvas = document.getElementById('particles');
            const ctx = canvas.getContext('2d');
            
            // Configurar canvas
            function resizeCanvas() {
                canvas.width = window.innerWidth;
                canvas.height = window.innerHeight;
            }
            
            resizeCanvas();
            window.addEventListener('resize', resizeCanvas);
            
            // Partículas
            const particles = [];
            const particleCount = 50;
            
            class Particle {
                constructor() {
                    this.x = Math.random() * canvas.width;
                    this.y = Math.random() * canvas.height;
                    this.vx = (Math.random() - 0.5) * 0.5;
                    this.vy = (Math.random() - 0.5) * 0.5;
                    this.size = Math.random() * 2 + 1;
                    this.opacity = Math.random() * 0.5 + 0.2;
                }
                
                update() {
                    this.x += this.vx;
                    this.y += this.vy;
                    
                    // Wrap around edges
                    if (this.x < 0) this.x = canvas.width;
                    if (this.x > canvas.width) this.x = 0;
                    if (this.y < 0) this.y = canvas.height;
                    if (this.y > canvas.height) this.y = 0;
                }
                
                draw() {
                    ctx.beginPath();
                    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
                    ctx.fillStyle = `rgba(0, 212, 255, ${this.opacity})`;
                    ctx.fill();
                }
            }
            
            // Crear partículas
            for (let i = 0; i < particleCount; i++) {
                particles.push(new Particle());
            }
            
            // Animación
            function animate() {
                ctx.clearRect(0, 0, canvas.width, canvas.height);
                
                particles.forEach(particle => {
                    particle.update();
                    particle.draw();
                });
                
                // Conectar partículas cercanas
                particles.forEach((p1, i) => {
                    particles.slice(i + 1).forEach(p2 => {
                        const dx = p1.x - p2.x;
                        const dy = p1.y - p2.y;
                        const distance = Math.sqrt(dx * dx + dy * dy);
                        
                        if (distance < 100) {
                            ctx.beginPath();
                            ctx.moveTo(p1.x, p1.y);
                            ctx.lineTo(p2.x, p2.y);
                            ctx.strokeStyle = `rgba(0, 212, 255, ${0.1 * (1 - distance / 100)})`;
                            ctx.stroke();
                        }
                    });
                });
                
                requestAnimationFrame(animate);
            }
            
            animate();
        }

        // Scroll suave
        function initSmoothScroll() {
            document.querySelectorAll('a[href^="#"]').forEach(anchor => {
                anchor.addEventListener('click', function (e) {
                    e.preventDefault();
                    const target = document.querySelector(this.getAttribute('href'));
                    if (target) {
                        target.scrollIntoView({
                            behavior: 'smooth',
                            block: 'start'
                        });
                    }
                });
            });
        }

        // Inicializar cuando el DOM esté listo
        document.addEventListener('DOMContentLoaded', function() {
            initTypewriter();
            initParticles();
            initSmoothScroll();
            
            // Mensaje de bienvenida en consola
            console.log('%c🤖 MechMind-dwv System Initialized', 'color: #00D4FF; font-size: 20px; font-weight: bold;');
            console.log('%c🚀 Welcome to the future of robotics!', 'color: #00FF88; font-size: 14px;');
        });
    </script>
</body>
</html>
