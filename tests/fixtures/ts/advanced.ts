import { EventEmitter } from "node:events";
import * as crypto from "node:crypto";

// ── Generic interface ───────────────────────────────────
export interface Repository<T> {
    findById(id: string): Promise<T | null>;
    findAll(): Promise<T[]>;
    save(entity: T): Promise<void>;
}

// ── Complex type alias ──────────────────────────────────
export type Result<T> = { ok: true; value: T } | { ok: false; error: string };

// ── Interface extending multiple interfaces ─────────────
interface Timestamped {
    createdAt: Date;
    updatedAt: Date;
}

interface Identifiable {
    id: string;
}

export interface Entity extends Identifiable, Timestamped {
    version: number;
}

// ── Abstract class with generics ────────────────────────
export abstract class BaseService<T extends Entity> {
    protected items: Map<string, T> = new Map();

    abstract validate(item: T): boolean;

    async find(id: string): Promise<T | undefined> {
        return this.items.get(id);
    }

    async save(item: T): Promise<void> {
        if (!this.validate(item)) {
            throw new Error("Validation failed");
        }
        this.items.set(item.id, item);
    }

    get count(): number {
        return this.items.size;
    }
}

// ── Concrete class with decorator-style patterns ────────
export class UserService extends BaseService<User> {
    private readonly maxUsers: number;

    constructor(maxUsers: number = 100) {
        super();
        this.maxUsers = maxUsers;
    }

    validate(item: User): boolean {
        return item.name.length > 0 && item.email.includes("@");
    }

    async findByEmail(email: string): Promise<User | undefined> {
        for (const user of this.items.values()) {
            if (user.email === email) {
                return user;
            }
        }
        return undefined;
    }
}

// ── Interface for the concrete type ─────────────────────
export interface User extends Entity {
    name: string;
    email: string;
    roles: string[];
}

// ── Exported const arrow function ───────────────────────
export const createUser = async (name: string, email: string): Promise<User> => {
    return {
        id: crypto.randomUUID(),
        name,
        email,
        roles: ["user"],
        version: 1,
        createdAt: new Date(),
        updatedAt: new Date(),
    };
};

// ── Generator function ──────────────────────────────────
export function* paginate<T>(items: T[], pageSize: number): Generator<T[]> {
    for (let i = 0; i < items.length; i += pageSize) {
        yield items.slice(i, i + pageSize);
    }
}

// ── Function with overloaded signatures ─────────────────
export function format(value: string): string;
export function format(value: number): string;
export function format(value: string | number): string {
    if (typeof value === "string") {
        return value.trim();
    }
    return value.toFixed(2);
}

// ── Namespace ───────────────────────────────────────────
export namespace Validators {
    export function isEmail(value: string): boolean {
        return value.includes("@");
    }

    export function isNonEmpty(value: string): boolean {
        return value.length > 0;
    }

    export type ValidationRule = (value: string) => boolean;
}

// ── Ambient module ──────────────────────────────────────
declare module "external-lib" {
    export function doSomething(opts: any): void;
    export const VERSION: string;
}
