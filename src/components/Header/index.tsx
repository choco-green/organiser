'use client';

import { usePathname } from 'next/navigation';

// Components
import AuthenticationSection from './AuthenticationSection';
import Logo from './Logo';
import SearchBar from './SearchBar';

export default function Header() {
  const isHome = usePathname() === '/';

  return (
    <header className="flex h-16 w-full items-center justify-between border-b px-8">
      <Logo />
      {!isHome && <SearchBar />}
      <AuthenticationSection />
    </header>
  );
}
