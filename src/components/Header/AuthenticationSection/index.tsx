'use client';

import { useUser } from '@auth0/nextjs-auth0/client';

// Components
import ProfileDropdown from './ProfileDropdown';

export default function AuthenticationSection() {
  const { user, isLoading } = useUser();

  return (
    <div className="flex h-full items-center gap-4">
      {user && !isLoading && user.picture && <ProfileDropdown image={user.picture} />}
      {!user && !isLoading && (
        <a href="/api/auth/login" className="flex h-8 items-center rounded border border-ctp-overlay0 bg-ctp-surface0 px-3">
          Get Started
        </a>
      )}
    </div>
  );
}
