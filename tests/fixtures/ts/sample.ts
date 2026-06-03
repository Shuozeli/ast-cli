export interface MyInterface {
    id: string;
}

export class MyClass implements MyInterface {
    id: string = "test";
    
    doThing(): void {
        console.log("thing");
    }
}

export function topLevelFunc() {
    return;
}
