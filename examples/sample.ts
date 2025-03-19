// Sample TypeScript file for testing syntax highlighting

// Import statements
import { Component } from '@angular/core';
import * as fs from 'fs';

// Interface definition
interface User {
    id: number;
    name: string;
    email?: string;
    roles: string[];
    isActive: boolean;
}

// Type definition
type UserRole = 'admin' | 'editor' | 'viewer';

// Enum definition
enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

// Class definition with decorator
@Component({
    selector: 'app-user',
    template: `<div>{{ user.name }}</div>`
})
class UserComponent {
    // Properties with type annotations
    private user: User;
    private role: UserRole = 'viewer';
    private status: Status = Status.Active;
    
    // Constructor
    constructor(user: User) {
        this.user = user;
    }
    
    // Method with return type
    public getUserInfo(): string {
        return `User: ${this.user.name}, Role: ${this.role}, Status: ${this.status}`;
    }
    
    // Async method
    public async fetchUserData(): Promise<User> {
        // Template string
        console.log(`Fetching data for user ${this.user.id}...`);
        
        // Promise and await
        return await new Promise<User>((resolve) => {
            setTimeout(() => {
                resolve(this.user);
            }, 1000);
        });
    }
    
    // Method with generic type
    public updateUser<T extends Partial<User>>(userData: T): void {
        this.user = { ...this.user, ...userData };
    }
}

// Function with rest parameters and arrow function
const calculateTotal = (...numbers: number[]): number => {
    return numbers.reduce((sum, num) => sum + num, 0);
};

// Object with type annotation
const config: { apiUrl: string; timeout: number } = {
    apiUrl: 'https://api.example.com',
    timeout: 5000
};

// Conditional (ternary) operator
const isAdmin = true ? 'Yes' : 'No';

// Numbers in different formats
const decimal = 123.456;
const hex = 0xABCDEF;
const binary = 0b1010;
const octal = 0o777;

// Null and undefined
const nullValue: null = null;
const undefinedValue: undefined = undefined;

// Array and tuple
const numbers: number[] = [1, 2, 3, 4, 5];
const tuple: [string, number] = ['hello', 42];

// Map and Set
const userMap = new Map<number, User>();
const roleSet = new Set<UserRole>();

// JSX/TSX example would be in a .tsx file
// This is just a comment showing what JSX would look like:
/*
import React from 'react';

const user = { name: 'John Doe', email: 'john@example.com' };

const element = (
    <div className="user-profile">
        <h1>{user.name}</h1>
        <p>Email: {user.email || 'No email provided'}</p>
        <button onClick={() => console.log('Button clicked')}>
            View Details
        </button>
    </div>
);
*/

// Export statement
export { UserComponent, User, UserRole, Status };