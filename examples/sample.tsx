// Sample TSX file for testing syntax highlighting

// Import statements
import React, { useState, useEffect } from 'react';

// Interface for component props
interface UserProfileProps {
    userId: number;
    showEmail: boolean;
    onUpdate?: (user: User) => void;
}

// Interface for user data
interface User {
    id: number;
    name: string;
    email: string;
    role: 'admin' | 'user';
    lastLogin: Date;
}

// Functional component with hooks
const UserProfile: React.FC<UserProfileProps> = ({ userId, showEmail, onUpdate }) => {
    // State hooks
    const [user, setUser] = useState<User | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    // Effect hook
    useEffect(() => {
        // Async function to fetch user data
        const fetchUser = async (): Promise<void> => {
            try {
                setLoading(true);
                // Simulated API call
                const response = await fetch(`https://api.example.com/users/${userId}`);
                
                if (!response.ok) {
                    throw new Error('Failed to fetch user data');
                }
                
                const userData: User = await response.json();
                setUser(userData);
                setError(null);
            } catch (err) {
                setError(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
                setUser(null);
            } finally {
                setLoading(false);
            }
        };

        fetchUser();
        
        // Cleanup function
        return () => {
            console.log('Component unmounted');
        };
    }, [userId]); // Dependency array

    // Event handler
    const handleUpdateClick = (): void => {
        if (user && onUpdate) {
            onUpdate(user);
        }
    };

    // Conditional rendering
    if (loading) {
        return <div className="loading">Loading user data...</div>;
    }

    if (error) {
        return <div className="error">{error}</div>;
    }

    if (!user) {
        return <div className="not-found">User not found</div>;
    }

    // JSX with conditional rendering, event binding, and props
    return (
        <div className="user-profile">
            <h1>{user.name}</h1>
            
            {showEmail && (
                <p className="email">
                    Email: <a href={`mailto:${user.email}`}>{user.email}</a>
                </p>
            )}
            
            <p className="role">
                Role: <span className={`role-badge ${user.role}`}>{user.role}</span>
            </p>
            
            <p className="login-time">
                Last login: {user.lastLogin.toLocaleString()}
            </p>
            
            <button 
                className="update-button"
                onClick={handleUpdateClick}
                disabled={!onUpdate}
            >
                Update User
            </button>
        </div>
    );
};

// Default props
UserProfile.defaultProps = {
    showEmail: true
};

// Export the component
export default UserProfile;