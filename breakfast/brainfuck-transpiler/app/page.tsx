"use client"

import React, { useState, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { InfoIcon } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"
import { Toaster } from "@/components/ui/toaster"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Copy, Check, ChevronsDown, ChevronsRight } from "lucide-react"

// Import the WebAssembly module
import { initializeWasm, transpile_brainfuck_to_c } from '@/lib/wasm-loader'

export default function BrainfuckTranspiler() {
  const [brainfuckCode, setBrainfuckCode] = useState(
    `++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`
  )
  const [cCode, setCCode] = useState("")
  const [activeTab, setActiveTab] = useState("brainfuck")
  const [copied, setCopied] = useState(false)
  const [isTranspiling, setIsTranspiling] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [wasmLoaded, setWasmLoaded] = useState(false)
  const { toast } = useToast()

  // Initialize WebAssembly module
  useEffect(() => {
    const loadWasm = async () => {
      try {
        await initializeWasm();
        setWasmLoaded(true);
        toast({
          title: "Ready",
          description: "WebAssembly module loaded successfully",
        });
      } catch (err) {
        console.error('Failed to load WebAssembly module:', err);
        setError('Failed to load WebAssembly module. Please refresh the page.');
        toast({
          title: "Error",
          description: "Failed to load WebAssembly module. Please refresh the page.",
          variant: "destructive",
        });
      }
    };
    
    loadWasm();
  }, [toast]);

  // This function uses the WebAssembly module to transpile Brainfuck to C
  const transpileToCWithWasm = async (code: string) => {
    try {
      setIsTranspiling(true);
      setError(null);

      if (!wasmLoaded) {
        throw new Error("WebAssembly module not loaded yet");
      }

      // Call the WebAssembly function to transpile the code
      const result = transpile_brainfuck_to_c(code);
      
      if (result.startsWith("Error:")) {
        throw new Error(result);
      }

      setCCode(result);
      setActiveTab("c");
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(errorMessage);
      throw err;
    } finally {
      setIsTranspiling(false);
    }
  };

  const handleTranspile = () => {
    if (!brainfuckCode.trim()) {
      toast({
        title: "Error",
        description: "Please enter some Brainfuck code to transpile",
        variant: "destructive",
      });
      return;
    }

    transpileToCWithWasm(brainfuckCode)
      .then(() => {
        toast({
          title: "Success",
          description: "Brainfuck transpiled to C successfully",
        });
      })
      .catch((err) => {
        toast({
          title: "Error",
          description: `Transpilation error: ${err instanceof Error ? err.message : String(err)}`,
          variant: "destructive",
        });
      });
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(cCode)
    toast({
      title: "Copied to clipboard",
      description: "The C code has been copied to your clipboard",
      variant: "success",
      duration: 3000,
    })
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      handleTranspile()
    }
  }

  return (
    <div className="container mx-auto py-8 px-4">
      <h1 className="text-3xl font-bold mb-6 text-center">Brainfuck to C Transpiler</h1>

      {!wasmLoaded && (
        <Alert className="mb-6">
          <InfoIcon className="h-4 w-4" />
          <AlertDescription>Loading WebAssembly module...</AlertDescription>
        </Alert>
      )}

      <div className="grid gap-6 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>Brainfuck Code</CardTitle>
            <CardDescription>Enter your Brainfuck code here</CardDescription>
          </CardHeader>
          <CardContent>
            <Textarea
              placeholder="Enter Brainfuck code here... (e.g. ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.)"
              className="font-mono h-[300px]"
              value={brainfuckCode}
              onChange={(e) => setBrainfuckCode(e.target.value)}
              onKeyDown={handleKeyDown}
            />
          </CardContent>
          <CardFooter>
            <Button onClick={handleTranspile} disabled={isTranspiling || !wasmLoaded} className="w-full">
              {isTranspiling ? "Transpiling..." : "Transpile to C"}
            </Button>
          </CardFooter>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>C Code</CardTitle>
            <CardDescription>Transpiled C code will appear here</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="relative">
              <Textarea
                className="font-mono h-[300px]"
                value={cCode}
                readOnly
                placeholder="Transpiled C code will appear here..."
              />
            </div>
          </CardContent>
          <CardFooter className="flex justify-between">
            <Button variant="outline" onClick={handleCopy} disabled={!cCode}>
              Copy to Clipboard
            </Button>
            <Button variant="outline" onClick={() => setCCode("")} disabled={!cCode}>
              Clear
            </Button>
          </CardFooter>
        </Card>
      </div>

      {error && (
        <Alert variant="destructive" className="mt-6">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      <div className="mt-8 p-4 bg-muted rounded-lg">
        <h2 className="text-xl font-semibold mb-2">How to integrate your Rust transpiler</h2>
        <ol className="list-decimal pl-5 space-y-2">
          <li>
            Compile your Rust transpiler to WebAssembly using <code>wasm-pack</code> or similar tools
          </li>
          <li>
            Import the compiled <code>.wasm</code> file in this application using{" "}
            <code>import wasmModule from './path-to-your-file.wasm?module'</code>
          </li>
          <li>Instantiate the WebAssembly module and call your transpiler function</li>
          <li>
            Replace the placeholder implementation in the <code>transpileToCWithWasm</code> function
          </li>
        </ol>
      </div>
      <Toaster />
    </div>
  )
}

