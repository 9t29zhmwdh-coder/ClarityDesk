import { create } from 'zustand'

export type Lang = 'en' | 'de'

const STORAGE_KEY = 'claritydesk_lang'

let currentLang: Lang = (localStorage.getItem(STORAGE_KEY) as Lang) || 'en'

export function getLang(): Lang {
  return currentLang
}

interface LangState {
  lang: Lang
  setLang: (l: Lang) => void
  toggle: () => void
}

export const useLangStore = create<LangState>((set) => ({
  lang: currentLang,
  setLang: (l) => {
    currentLang = l
    localStorage.setItem(STORAGE_KEY, l)
    set({ lang: l })
  },
  toggle: () => {
    const next: Lang = currentLang === 'en' ? 'de' : 'en'
    currentLang = next
    localStorage.setItem(STORAGE_KEY, next)
    set({ lang: next })
  },
}))

const translations = {
  en: {
    navDashboard: 'Dashboard', navAnalysis: 'Analysis', navSettings: 'Settings',

    appTagline: 'Universal Display Interpreter: offline, local AI',
    consentTitle: 'Permission Required',
    consentDesc: 'ClarityDesk needs your consent to capture screen content. No data is stored or transmitted; everything is processed locally.',
    consentAllow: 'Allow Screen Analysis',

    localAiTitle: 'Local AI (Ollama)', connected: 'Connected', offline: 'Offline',
    startOllamaHintPrefix: 'Start Ollama with', startOllamaHintSuffix: 'to enable AI analysis.',

    analysisModeLabel: 'Analysis Mode',
    modeLanguageLabel: 'Language', modeLanguageDesc: 'Translate text to your target language',
    modeDevLabel: 'Dev Mode', modeDevDesc: 'Explain code, analyze logs & terminal',
    modeSmartLabel: 'Smart', modeSmartDesc: 'Auto-detect content and apply best mode',

    capturing: 'Capturing…', analyzing: 'Analyzing…', captureAnalyze: 'Capture & Analyze',

    hotkeysLabel: 'Hotkeys',
    hkCapture: 'Capture & Analyze', hkDev: 'Dev Mode', hkSmart: 'Smart Mode', hkReanalyze: 'Re-analyze',

    lastCapture: 'Last Capture', blocksFound: 'Blocks Found', noData: 'N/A',

    noCaptureYet: 'No capture yet. Use Dashboard to capture the screen.',
    analyzed: 'Analyzed', original: 'Original', reanalyze: 'Re-analyze',
    analyzingWithAi: 'Analyzing content with local AI…',
    noBlocksDetected: 'No text blocks detected.',
    tryDifferentRegion: 'Try a different region or check OCR language settings.',
    screenCapturedPrompt: 'Screen captured. Click Re-analyze or use the Dashboard to analyze.',
    blockSingular: '{{n}} block', blockPlural: '{{n}} blocks',

    settingsTitle: 'Settings', reset: 'Reset', save: 'Save', saved: 'Saved',
    loadingSettings: 'Loading settings…',
    ollamaHost: 'Ollama Host', model: 'Model',
    analysisSection: 'Analysis', defaultMode: 'Default Mode',
    modeSmartOption: 'Smart (auto-detect)', modeLanguageOption: 'Language (translate)', modeDevOption: 'Dev (explain code/logs)',
    targetLanguage: 'Target Language',
    ocrSection: 'OCR', ocrLanguage: 'OCR Language',
    ocrHintPrefix: 'Tesseract language codes separated by', ocrHintExample: 'Example:',
    hkStartStop: 'Start / Stop', hkReanalyzeLabel: 'Re-Analyze',
    privacySection: 'Privacy',
    privStoreCaptures: 'Store captured images on disk',
    privStoreResults: 'Store analysis results on disk',
    privShowConsent: 'Show consent dialog on startup',
    privacyNote: 'ClarityDesk processes everything locally. No data leaves your device. Tesseract and Ollama run entirely offline.',
  },
  de: {
    navDashboard: 'Übersicht', navAnalysis: 'Analyse', navSettings: 'Einstellungen',

    appTagline: 'Universeller Bildschirm-Interpreter: offline, lokale KI',
    consentTitle: 'Berechtigung erforderlich',
    consentDesc: 'ClarityDesk benötigt deine Zustimmung, um Bildschirminhalte zu erfassen. Es werden keine Daten gespeichert oder übertragen; alles wird lokal verarbeitet.',
    consentAllow: 'Bildschirmanalyse erlauben',

    localAiTitle: 'Lokale KI (Ollama)', connected: 'Verbunden', offline: 'Offline',
    startOllamaHintPrefix: 'Starte Ollama mit', startOllamaHintSuffix: 'um die KI-Analyse zu aktivieren.',

    analysisModeLabel: 'Analysemodus',
    modeLanguageLabel: 'Sprache', modeLanguageDesc: 'Übersetzt Text in deine Zielsprache',
    modeDevLabel: 'Dev-Modus', modeDevDesc: 'Erklärt Code, analysiert Logs & Terminal',
    modeSmartLabel: 'Smart', modeSmartDesc: 'Erkennt Inhalt automatisch und wendet den besten Modus an',

    capturing: 'Erfasse…', analyzing: 'Analysiere…', captureAnalyze: 'Erfassen & Analysieren',

    hotkeysLabel: 'Tastenkürzel',
    hkCapture: 'Erfassen & Analysieren', hkDev: 'Dev-Modus', hkSmart: 'Smart-Modus', hkReanalyze: 'Neu analysieren',

    lastCapture: 'Letzte Erfassung', blocksFound: 'Blöcke gefunden', noData: 'k. A.',

    noCaptureYet: 'Noch keine Erfassung. Nutze die Übersicht, um den Bildschirm zu erfassen.',
    analyzed: 'Analysiert', original: 'Original', reanalyze: 'Neu analysieren',
    analyzingWithAi: 'Analysiere Inhalt mit lokaler KI…',
    noBlocksDetected: 'Keine Textblöcke erkannt.',
    tryDifferentRegion: 'Versuche einen anderen Bereich oder prüfe die OCR-Spracheinstellungen.',
    screenCapturedPrompt: 'Bildschirm erfasst. Klicke auf Neu analysieren oder nutze die Übersicht.',
    blockSingular: '{{n}} Block', blockPlural: '{{n}} Blöcke',

    settingsTitle: 'Einstellungen', reset: 'Zurücksetzen', save: 'Speichern', saved: 'Gespeichert',
    loadingSettings: 'Lade Einstellungen…',
    ollamaHost: 'Ollama-Host', model: 'Modell',
    analysisSection: 'Analyse', defaultMode: 'Standardmodus',
    modeSmartOption: 'Smart (automatisch)', modeLanguageOption: 'Sprache (Übersetzung)', modeDevOption: 'Dev (Code/Logs erklären)',
    targetLanguage: 'Zielsprache',
    ocrSection: 'OCR', ocrLanguage: 'OCR-Sprache',
    ocrHintPrefix: 'Tesseract-Sprachcodes getrennt durch', ocrHintExample: 'Beispiel:',
    hkStartStop: 'Start / Stopp', hkReanalyzeLabel: 'Neu analysieren',
    privacySection: 'Datenschutz',
    privStoreCaptures: 'Erfasste Bilder auf der Festplatte speichern',
    privStoreResults: 'Analyseergebnisse auf der Festplatte speichern',
    privShowConsent: 'Zustimmungsdialog beim Start anzeigen',
    privacyNote: 'ClarityDesk verarbeitet alles lokal. Es verlassen keine Daten dein Gerät. Tesseract und Ollama laufen vollständig offline.',
  },
} as const

type TranslationKey = keyof typeof translations.en

function interpolate(str: string, vars?: Record<string, string | number>): string {
  if (!vars) return str
  return str.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''))
}

export function t(key: TranslationKey, vars?: Record<string, string | number>): string {
  const str = translations[currentLang][key] ?? key
  return interpolate(str, vars)
}

export function useT() {
  const lang = useLangStore((s) => s.lang)
  return (key: TranslationKey, vars?: Record<string, string | number>) => {
    const str = translations[lang][key] ?? key
    return interpolate(str, vars)
  }
}
