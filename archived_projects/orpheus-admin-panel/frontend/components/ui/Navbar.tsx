"use client";

import Link from 'next/link';
import { Github, Menu } from 'lucide-react';
import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { motion } from 'framer-motion';

/**
 * GitHub Star Button Component
 */
function GitHubStarButton({ owner, repo }: { owner: string; repo: string }) {
  const [stars, setStars] = useState<number | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const fetchStars = async () => {
      setLoading(true);
      try {
        const response = await axios.get(`http://127.0.0.1:8080/github/stars/${owner}/${repo}`);
        if (response.data && response.data.success) {
          setStars(response.data.data.stars);
        }
      } catch (error) {
        console.error('Error fetching stars:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchStars();
  }, [owner, repo]);

  const displayStars = loading ? '...' : stars !== null ? stars.toString() : 'Star';

  return (
    <Link
      href={`https://github.com/${owner}/${repo}`}
      target="_blank"
      className="flex items-center gap-2 px-3 py-1.5 rounded-full border border-white/10 bg-white/5 hover:bg-white/10 hover:border-primary/50 transition-all text-sm text-muted-foreground hover:text-foreground"
    >
      <Github size={16} />
      <span className="font-medium">{displayStars}</span>
    </Link>
  );
}

const Navbar = () => {
  return (
    <motion.header
      initial={{ y: -100, opacity: 0 }}
      animate={{ y: 0, opacity: 1 }}
      transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
      className="fixed top-0 left-0 right-0 z-50 border-b border-white/5 bg-black/50 backdrop-blur-md"
    >
      <nav className="container mx-auto px-6 h-16 flex justify-between items-center">
        {/* Left: Logo */}
        <div className="flex items-center gap-8">
          <Link href="/" className="flex items-center gap-2 group">
            <div className="w-6 h-6 bg-primary rounded-full blur-[1px] opacity-80 group-hover:opacity-100 transition-opacity" />
            <span className="font-bold text-lg tracking-tight text-foreground">Orpheus</span>
          </Link>

          {/* Desktop Links */}
          <ul className="hidden md:flex items-center gap-6 text-sm font-medium text-muted-foreground">
            <li><Link href="/features" className="hover:text-primary transition-colors">Features</Link></li>
            <li><Link href="/pricing" className="hover:text-primary transition-colors">Pricing</Link></li>
            <li><Link href="/docs" className="hover:text-primary transition-colors">Docs</Link></li>
            <li><Link href="/blog" className="hover:text-primary transition-colors">Blog</Link></li>
          </ul>
        </div>

        {/* Right: Actions */}
        <div className="hidden md:flex items-center gap-4">
          <GitHubStarButton owner="KayanoLiam" repo="Orpheus" />

          <Link href="/login" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
            Sign in
          </Link>

          <Link
            href="/login"
            className="text-sm font-bold px-4 py-2 rounded-full bg-primary text-black hover:bg-primary/90 transition-all shadow-[0_0_15px_-5px_rgba(var(--primary),0.5)]"
          >
            Get Started
          </Link>
        </div>

        {/* Mobile Menu Button */}
        <div className="md:hidden text-muted-foreground hover:text-foreground cursor-pointer">
          <Menu size={24} />
        </div>
      </nav>
    </motion.header>
  );
};

export default Navbar;