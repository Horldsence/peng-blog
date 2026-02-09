/**
 * Liquid Glass Background Component
 *
 * A dynamic liquid glass refraction effect based on Three.js and html2canvas.
 * This component creates a full-screen liquid glass effect with particle physics simulation.
 *
 * Features:
 * - Particle-based physics simulation with attraction/repulsion forces
 * - Metaball algorithm for morphology
 * - GPU-accelerated refraction and dispersion rendering
 * - Dynamic background capture for real-time refraction
 * - Mouse interaction (drag particles)
 *
 * Performance Optimizations:
 * - Reduced particle count (10 particles instead of 15)
 * - Configurable capture frequency (every 3 frames instead of 2)
 * - Efficient shader-based rendering
 */

import { useEffect, useRef } from 'react';
import * as THREE from 'three';
import html2canvas from 'html2canvas';

// Configuration
const CONFIG = {
  // Particle settings
  particleNum: 10, // Reduced from 15 for better performance
  particleRenderRadius: 50,

  // Refraction strength (RGB dispersion effect)
  refractionStrengthR: '470.',
  refractionStrengthG: '500.',
  refractionStrengthB: '530.',

  // Liquid shape parameters
  dropletBendingParameter: '2.', // >= 0
  dropletEdgeBendingParameter: '5.', // >= 0
  dropletEdgeSharpnessParameter: '20.', // >= 0

  // Physics parameters
  dt: 0.5, // Time step
  targetDistance: 40, // Repulsion distance threshold
  attractionPeak: 100, // Attraction peak distance
  attractionEnd: 200, // Attraction zero distance
  forceScale1: 2, // Repulsion force scale
  forceScale2: 0.5, // Attraction force scale
  damping: 0.03, // Damping factor
  viscosityStrength: 0.03, // Viscosity strength
  viscosityEnd: 100, // Viscosity end distance
  boundaryMargin: 50, // Boundary margin

  // Drag interaction
  dragStrength: 0.2,
  dragViscosityFactor: 10,

  // Performance
  captureInterval: 3, // Capture background every N frames
};

interface Particles {
  location: number[][];
  velocity: number[][];
}

export function LiquidGlassBackground() {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const particlesRef = useRef<Particles | null>(null);
  const dragStateRef = useRef<any>(null);
  const animationRef = useRef<number | null>(null);

  useEffect(() => {
    if (!containerRef.current || !canvasRef.current) return;

    const container = containerRef.current;
    const canvas = canvasRef.current;
    const liquidCanvas = document.createElement('canvas');
    liquidCanvas.id = 'liquid-canvas';
    liquidCanvas.style.position = 'absolute';
    liquidCanvas.style.top = '0';
    liquidCanvas.style.left = '0';
    liquidCanvas.style.width = '100%';
    liquidCanvas.style.height = '100%';
    container.appendChild(liquidCanvas);

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // Initialize particles
    const initParticles = (): Particles => {
      const width = window.innerWidth;
      const height = window.innerHeight;

      return {
        location: Array.from({ length: CONFIG.particleNum }, () => [
          (0.5 + (Math.random() - 0.5) * 0.3) * width,
          (0.5 + (Math.random() - 0.5) * 0.3) * height,
        ]),
        velocity: Array.from({ length: CONFIG.particleNum }, () => [0, 0]),
      };
    };

    particlesRef.current = initParticles();
    dragStateRef.current = {
      active: false,
      particleIndex: undefined,
      offset: undefined,
      target: undefined,
    };

    // Physics simulation step
    const oneStep = () => {
      if (!particlesRef.current) return;
      const particles = particlesRef.current;
      const dragState = dragStateRef.current;

      const bodyWidth = window.innerWidth - CONFIG.boundaryMargin;
      const bodyHeight = window.innerHeight - CONFIG.boundaryMargin;

      // Apply damping
      for (let i = 0; i < CONFIG.particleNum; i++) {
        particles.velocity[i][0] *= 1 - CONFIG.damping * CONFIG.dt;
        particles.velocity[i][1] *= 1 - CONFIG.damping * CONFIG.dt;
      }

      // Particle interactions
      for (let i = 0; i < CONFIG.particleNum; i++) {
        const posA = particles.location[i];

        for (let j = i + 1; j < CONFIG.particleNum; j++) {
          const posB = particles.location[j];
          const dx = posB[0] - posA[0];
          const dy = posB[1] - posA[1];
          const dist = Math.sqrt(dx * dx + dy * dy);

          // Viscosity
          if (dist < CONFIG.viscosityEnd) {
            let strength = CONFIG.viscosityStrength * (1 - dist / CONFIG.viscosityEnd);
            if (
              dragState.active &&
              (dragState.particleIndex === i || dragState.particleIndex === j)
            ) {
              strength *= CONFIG.dragViscosityFactor;
            }
            const impactX =
              (particles.velocity[j][0] - particles.velocity[i][0]) * strength * CONFIG.dt;
            const impactY =
              (particles.velocity[j][1] - particles.velocity[i][1]) * strength * CONFIG.dt;
            particles.velocity[i][0] += impactX;
            particles.velocity[i][1] += impactY;
            particles.velocity[j][0] -= impactX;
            particles.velocity[j][1] -= impactY;
          }

          // Distance-based forces
          if (dist < CONFIG.targetDistance) {
            // Repulsion
            const force = (CONFIG.forceScale1 * (CONFIG.targetDistance - dist)) / dist;
            const impactX = -dx * force * CONFIG.dt;
            const impactY = -dy * force * CONFIG.dt;
            particles.velocity[i][0] += impactX;
            particles.velocity[i][1] += impactY;
            particles.velocity[j][0] -= impactX;
            particles.velocity[j][1] -= impactY;
          } else if (dist < CONFIG.attractionEnd) {
            // Attraction
            let force: number;
            if (dist < CONFIG.attractionPeak) {
              force = (CONFIG.forceScale2 * (dist - CONFIG.targetDistance)) / CONFIG.attractionPeak;
            } else {
              force =
                (CONFIG.forceScale2 * (CONFIG.attractionEnd - dist)) /
                (CONFIG.attractionEnd - CONFIG.attractionPeak);
            }
            const impactX = ((dx * force) / dist) * CONFIG.dt;
            const impactY = ((dy * force) / dist) * CONFIG.dt;
            particles.velocity[i][0] += impactX;
            particles.velocity[i][1] += impactY;
            particles.velocity[j][0] -= impactX;
            particles.velocity[j][1] -= impactY;
          }
        }
      }

      // Mouse drag
      if (dragState.active) {
        const dx = dragState.target[0] - particles.location[dragState.particleIndex][0];
        const dy = dragState.target[1] - particles.location[dragState.particleIndex][1];
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist < CONFIG.particleRenderRadius * 2) {
          particles.velocity[dragState.particleIndex][0] *= 0.8;
          particles.velocity[dragState.particleIndex][1] *= 0.8;
        }

        particles.velocity[dragState.particleIndex][0] += dx * CONFIG.dragStrength;
        particles.velocity[dragState.particleIndex][1] += dy * CONFIG.dragStrength;
      }

      // Update positions and handle boundaries
      for (let i = 0; i < CONFIG.particleNum; i++) {
        const pos = particles.location[i];
        const vel = particles.velocity[i];

        pos[0] += vel[0] * CONFIG.dt;
        pos[1] += vel[1] * CONFIG.dt;

        // Boundary constraints (elastic collision)
        if (pos[0] < CONFIG.boundaryMargin) {
          pos[0] = CONFIG.boundaryMargin;
          vel[0] *= -0.5;
        } else if (pos[0] > bodyWidth) {
          pos[0] = bodyWidth;
          vel[0] *= -0.5;
        }

        if (pos[1] < CONFIG.boundaryMargin) {
          pos[1] = CONFIG.boundaryMargin;
          vel[1] *= -0.5;
        } else if (pos[1] > bodyHeight) {
          pos[1] = bodyHeight;
          vel[1] *= -0.5;
        }
      }
    };

    // Initialize Three.js
    const initThreeJS = () => {
      const width = window.innerWidth;
      const height = window.innerHeight;

      const renderer = new THREE.WebGLRenderer({
        antialias: false,
        canvas: liquidCanvas,
      });
      renderer.setSize(width, height);
      renderer.setClearColor(0x000000, 0);

      const scene = new THREE.Scene();
      const camera = new THREE.OrthographicCamera(
        -width / 2,
        width / 2,
        height / 2,
        -height / 2,
        1,
        1000
      );
      camera.position.z = 100;

      const geometry = new THREE.PlaneGeometry(width, height);

      // Liquid shader
      const liquidShader = {
        uniforms: {
          tDiffuse: { value: null },
          resolution: { value: new THREE.Vector2() },
          direction: { value: new THREE.Vector2() },
          radius: { value: CONFIG.particleRenderRadius },
          particles: { value: [] as THREE.Vector2[] },
        },
        vertexShader: `
          varying vec2 vUv;
          void main() {
            vUv = uv;
            gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
          }
        `,
        fragmentShader: `
          uniform sampler2D tDiffuse;
          uniform vec2 resolution;
          uniform vec2 direction;
          uniform float radius;
          uniform vec2 particles[${CONFIG.particleNum}];
          varying vec2 vUv;

          float gaussian(vec2 x, float sigma) {
            return exp(-dot(x, x) / (2.0 * sigma * sigma));
          }

          void main() {
            vec2 pixelSize = vec2(1.0) / resolution;

            float sum = 0.;
            vec2 normal = vec2(0., 0.);
            for (int i = 0; i < ${CONFIG.particleNum}; i ++) {
              vec2 d = vUv * resolution - particles[i];
              float weight = gaussian(d, radius);
              sum += weight;
              normal += d * weight;
            }
            normal *= 0.01 / sum;
            normal *= pow(length(normal), ${CONFIG.dropletBendingParameter});
            normal *= 1. + ${CONFIG.dropletEdgeBendingParameter} * exp(- ${CONFIG.dropletEdgeSharpnessParameter} * (sum - 0.7));

            if (sum > 0.7) {
              // Dispersion refraction
              gl_FragColor = vec4(
                texture2D(tDiffuse, vUv - normal * pixelSize * ${CONFIG.refractionStrengthR}).x,
                texture2D(tDiffuse, vUv - normal * pixelSize * ${CONFIG.refractionStrengthG}).y,
                texture2D(tDiffuse, vUv - normal * pixelSize * ${CONFIG.refractionStrengthB}).z,
                1.
              );
              gl_FragColor.xyz = gl_FragColor.xyz * 0.95;
            } else
              gl_FragColor = texture2D(tDiffuse, vUv);
          }
        `,
      };

      const material = new THREE.ShaderMaterial({
        uniforms: THREE.UniformsUtils.clone(liquidShader.uniforms),
        vertexShader: liquidShader.vertexShader,
        fragmentShader: liquidShader.fragmentShader,
      });

      material.uniforms.resolution.value.set(width, height);
      material.uniforms.direction.value.set(0.0, 1.0);

      const mesh = new THREE.Mesh(geometry, material);

      const render = (texture: THREE.Texture) => {
        material.uniforms.tDiffuse.value = texture;
        scene.add(mesh);
        renderer.render(scene, camera);
        scene.remove(mesh);
      };

      return { renderer, material, render };
    };

    const { renderer, material, render } = initThreeJS();

    // Capture background and create texture
    let texture: THREE.Texture | null = null;

    const captureAndProcess = () => {
      const originalBg = document.body.style.backgroundColor;
      document.body.style.backgroundColor = 'white';

      const width = window.innerWidth;
      const height = window.innerHeight;

      html2canvas(document.body, {
        scale: 1,
        logging: false,
        useCORS: true,
        allowTaint: false,
        backgroundColor: '#FFFFFF',
        windowWidth: width,
        windowHeight: height,
        width: width,
        height: height,
        ignoreElements: (element) => element === container,
      })
        .then((capturedCanvas) => {
          document.body.style.backgroundColor = originalBg;

          texture = new THREE.CanvasTexture(capturedCanvas);
          texture.minFilter = THREE.LinearFilter;
          texture.magFilter = THREE.LinearFilter;

          render(texture);
        })
        .catch((err) => {
          console.error('Failed to capture content:', err);
          document.body.style.backgroundColor = originalBg;
        });
    };

    // Mouse events
    const handleMouseDown = (e: MouseEvent) => {
      if (!particlesRef.current) return;
      const particles = particlesRef.current;
      const mouseX = e.clientX;
      const mouseY = e.clientY;
      const threshold = CONFIG.particleRenderRadius;

      let closestDist = Infinity;
      let closestIndex: number | undefined = undefined;

      particles.location.forEach((pos, i) => {
        const dx = pos[0] - mouseX;
        const dy = pos[1] - mouseY;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist < threshold && dist < closestDist) {
          closestDist = dist;
          closestIndex = i;
        }
      });

      if (closestIndex !== undefined) {
        dragStateRef.current.particleIndex = closestIndex;
        dragStateRef.current.offset = [
          particles.location[closestIndex][0] - mouseX,
          particles.location[closestIndex][1] - mouseY,
        ];
        dragStateRef.current.target = [...particles.location[closestIndex]];
        dragStateRef.current.active = true;
      }
    };

    const handleMouseMove = (e: MouseEvent) => {
      if (!dragStateRef.current.active) return;
      const mouseX = e.clientX;
      const mouseY = e.clientY;
      const targetX = mouseX + dragStateRef.current.offset[0];
      const targetY = mouseY + dragStateRef.current.offset[1];
      dragStateRef.current.target = [targetX, targetY];
    };

    const handleMouseUp = () => {
      dragStateRef.current.active = false;
    };

    canvas.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);

    // Resize handler
    const handleResize = () => {
      const width = window.innerWidth;
      const height = window.innerHeight;
      canvas.width = width;
      canvas.height = height;
      renderer.setSize(width, height);
      material.uniforms.resolution.value.set(width, height);
      captureAndProcess();
    };
    window.addEventListener('resize', handleResize);

    // Initial capture
    const preloadImages = () => {
      return new Promise<void>((resolve) => {
        const images = document.querySelectorAll('img');
        let loadedCount = 0;

        if (images.length === 0) {
          resolve();
          return;
        }

        images.forEach((img) => {
          if (img.complete) {
            loadedCount++;
          } else {
            img.onload = () => {
              loadedCount++;
              if (loadedCount === images.length) resolve();
            };
            img.onerror = () => {
              loadedCount++;
              if (loadedCount === images.length) resolve();
            };
          }
        });

        if (loadedCount === images.length) resolve();
      });
    };

    // Animation loop
    let step = 0;
    const animationLoop = () => {
      oneStep();

      if (particlesRef.current) {
        material.uniforms.particles.value = particlesRef.current.location.map(
          (e) => new THREE.Vector2(e[0], window.innerHeight - e[1])
        );
      }

      if (step % CONFIG.captureInterval === 0) {
        captureAndProcess();
      } else if (texture) {
        render(texture);
      }

      step++;
      animationRef.current = requestAnimationFrame(animationLoop);
    };

    // Start animation
    preloadImages().then(() => {
      captureAndProcess();
      animationLoop();
    });

    // Cleanup
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
      canvas.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
      window.removeEventListener('resize', handleResize);
      container.removeChild(liquidCanvas);
      renderer.dispose();
      material.dispose();
    };
  }, []);

  return (
    <div
      ref={containerRef}
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        width: '100vw',
        height: '100vh',
        zIndex: 9999,
        pointerEvents: 'none',
      }}
    >
      <canvas
        ref={canvasRef}
        style={{
          position: 'fixed',
          top: 0,
          left: 0,
          width: '100%',
          height: '100%',
          zIndex: 10000,
          pointerEvents: 'auto',
        }}
      />
    </div>
  );
}
